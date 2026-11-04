//! OpenAI 호환 `/chat/completions` 스트리밍 클라이언트입니다.
//!
//! 엔드포인트 주소·종류·키·시간 설정을 갖고, 요청마다 본문만 받습니다.
//! 헤더 전 연결 오류를 세부 6종으로 나누는 일은 `connect_classify`가 맡습니다.

use std::fmt;
use std::sync::Arc;
use std::time::Instant;

use crate::config::{EndpointKind, HttpSettings};
use crate::llm::client::{
    CancellationToken, ChatRequest, ChatStream, LlmClient, StreamTiming, TimeoutSegment,
    TransportError,
};
use crate::llm::decode::StreamDecoder;
use crate::llm::error::redact::summarize_stream_error;
use crate::llm::error::scrub::SecretScrub;
use crate::llm::request_body::build_chat_completions_body;
use crate::llm::transport::connect_classify::classify_connect_error;
use crate::llm::transport::stream_state::StreamState;
use crate::secrets::SecretValue;

/// OpenAI 호환 `/chat/completions` 스트리밍 클라이언트입니다.
///
/// 엔드포인트 주소·종류·키·시간 설정을 갖고, 요청마다 본문만 받습니다.
pub struct OpenAiCompatClient {
    base_url: SecretValue,
    kind: EndpointKind,
    api_key: Option<SecretValue>,
    http: HttpSettings,
    /// HTTP 전송용 클라이언트입니다. 공개 타입으로 내보내지 않습니다.
    http_client: Arc<reqwest::Client>,
}

impl OpenAiCompatClient {
    /// 엔드포인트 정보와 시간 설정으로 클라이언트를 만듭니다.
    ///
    /// 연결 시간 초과는 HTTP 라이브러리에 맡깁니다. 헤더 대기·유휴는 여기서 따로 잽니다.
    pub fn new(
        base_url: SecretValue,
        kind: EndpointKind,
        api_key: Option<SecretValue>,
        http: HttpSettings,
    ) -> Result<Self, TransportError> {
        let http_client = reqwest::Client::builder()
            .connect_timeout(http.connect_seconds)
            .build()
            .map_err(|_| TransportError::ClientBuild)?;
        Ok(Self {
            base_url,
            kind,
            api_key,
            http,
            http_client: Arc::new(http_client),
        })
    }

    /// 이 요청에서 알고 있는 키·base_url로 가릴 가림 도구입니다(D28).
    fn scrub(&self) -> SecretScrub {
        SecretScrub::new(self.api_key.as_ref(), &self.base_url)
    }

    /// 응답 헤더를 정해진 시간 안에 받습니다. 보내기·헤더 대기·취소를 한 자리에서 경합합니다.
    async fn wait_for_headers(
        &self,
        builder: reqwest::RequestBuilder,
        cancel: &CancellationToken,
    ) -> Result<reqwest::Response, TransportError> {
        let send = builder.send();
        // 보내기가 끝나기를 기다립니다. 이 자리에서 취소를 함께 봅니다(취소 시 연결 정리는 아래 스트림에서).
        tokio::select! {
            biased;
            () = cancel.cancelled() => Err(TransportError::Cancelled),
            () = tokio::time::sleep(self.http.header_wait_seconds) => Err(self.timeout_error(TimeoutSegment::HeaderWait)),
            result = send => result.map_err(|e| self.connect_error(&e)),
        }
    }

    /// 서버가 정상이 아닌 상태로 준 응답을 오류로 바꿉니다.
    ///
    /// 본문은 어림 가리기 뒤 알고 있는 키·호스트로 한 번 더 가립니다(D28). 읽기도 유휴·취소와
    /// 경합해 취소되면 취소 오류를 내고, 헤더는 보냈지만 본문을 끝까지 안 보내는 서버에 멈춰 있지 않습니다.
    async fn server_status_error(
        &self,
        response: reqwest::Response,
        cancel: &CancellationToken,
    ) -> TransportError {
        let status = response.status().as_u16();
        let body = tokio::select! {
            result = response.bytes() => {
                match result {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                    Err(error) => summarize_stream_error(&error.to_string()),
                }
            }
            // 취소는 상태 코드 요약을 내려주지 않고 취소 오류로 바로 끝냅니다.
            () = cancel.cancelled() => return TransportError::Cancelled,
            () = tokio::time::sleep(self.http.stream_idle_seconds) => "서버가 오류 본문을 보내지 않았습니다".to_owned(),
        };
        TransportError::ServerStatus {
            status,
            body_summary: self.scrub().describe(&body),
        }
    }

    /// 헤더 전 연결 실패를 세부 6종으로 나눕니다(D26).
    ///
    /// 라이브러리가 주소를 오류 문자열에 넣을 수 있어 원문 텍스트는 버리고,
    /// Display·Debug에는 종류와 연결 설정 초 값만 담습니다.
    fn connect_error(&self, error: &reqwest::Error) -> TransportError {
        TransportError::ConnectFailed {
            kind: classify_connect_error(error),
            connect_seconds: self.http.connect_seconds.as_secs(),
        }
    }

    /// 구간 이름과 설정 초 값으로 시간 초과 오류를 만듭니다.
    fn timeout_error(&self, segment: TimeoutSegment) -> TransportError {
        let seconds = match segment {
            TimeoutSegment::HeaderWait => self.http.header_wait_seconds,
            TimeoutSegment::StreamIdle => self.http.stream_idle_seconds,
        };
        TransportError::Timeout {
            segment,
            seconds: seconds.as_secs(),
        }
    }
}

impl LlmClient for OpenAiCompatClient {
    async fn stream_chat(
        &self,
        request: ChatRequest,
        cancel: CancellationToken,
    ) -> Result<ChatStream, TransportError> {
        // 호출 전 취소를 바로 알립니다.
        if cancel.is_cancelled() {
            return Err(TransportError::Cancelled);
        }

        let url = format!("{}/chat/completions", self.base_url.reveal());
        let body = build_chat_completions_body(self.kind, &request);
        let request_started = Instant::now();

        let mut builder = self.http_client.post(url).json(&body);

        // 키가 있을 때만 Bearer 헤더를 붙입니다. 본문·URL·다른 헤더에는 넣지 않습니다.
        // 헤더 값은 HTTP 라이브러리 디버그·추적 로그에서도 가립니다.
        if let Some(api_key) = &self.api_key {
            let mut value =
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key.reveal()))
                    .map_err(|_| TransportError::ClientBuild)?;
            // 라이브러리 내부 로그에서 키가 드러나지 않게 민감 헤더로 표시한다(보안 기본).
            value.set_sensitive(true);
            builder = builder.header(reqwest::header::AUTHORIZATION, value);
        }

        let response = self.wait_for_headers(builder, &cancel).await?;
        if !response.status().is_success() {
            return Err(self.server_status_error(response, &cancel).await);
        }

        Ok(ChatStream::new(Box::new(StreamState {
            response,
            decoder: Some(StreamDecoder::new()),
            cancel,
            idle: self.http.stream_idle_seconds,
            timing: StreamTiming {
                request_started,
                first_token: None,
                last_token: None,
            },
            pending: std::collections::VecDeque::new(),
            upstream_done: false,
            usage: None,
            end_pending: false,
            failed: false,
        })))
    }
}

impl fmt::Debug for OpenAiCompatClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAiCompatClient")
            .field("base_url", &self.base_url)
            .field("kind", &self.kind)
            .field("api_key", &self.api_key)
            .field("http", &self.http)
            .finish_non_exhaustive()
    }
}
