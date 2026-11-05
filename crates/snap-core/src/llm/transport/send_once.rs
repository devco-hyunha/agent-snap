//! 전송 한 번(요청 보내기→헤더 받기) 책임입니다.
//!
//! 보내기와 헤더 대기·취소 경합, 정상 아닌 응답을 오류로 바꾸는 읽기까지 이 파일에서 합니다.
//! 헤더 전 연결 오류를 세부 6종으로 나누는 일은 `connect_classify`가 맡습니다(02-5).
//! 재시도 판단 반복과 대기·스트림 열기는 각각 `retry_loop`·`stream_chat`·`open_stream`이 맡습니다.

use crate::llm::client::{CancellationToken, ChatRequest, TimeoutSegment, TransportError};
use crate::llm::error::redact::summarize_stream_error;
use crate::llm::request_body::build_chat_completions_body;
use crate::llm::transport::connect_classify::classify_connect_error;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 응답 헤더까지 받아 오는 전송 한 번입니다.
    ///
    /// 정상 상태가 아니면 오류로 바꿉니다. 실패 봉지에는 응답이 준 `Retry-After` 헤더 값만 담습니다.
    /// 헤더 원문은 키·호스트와 달리 공개 오류 타입에는 넘기지 않고 판단에만 씁니다(02-6 설계 요약).
    pub(super) async fn send_once(
        &self,
        request: &ChatRequest,
        cancel: &CancellationToken,
    ) -> Result<reqwest::Response, SendFailure> {
        let url = format!("{}/chat/completions", self.base_url.reveal());
        let body = build_chat_completions_body(self.kind, request);
        let mut builder = self.http_client.post(url).json(&body);

        // 키가 있을 때만 Bearer 헤더를 붙입니다. 본문·URL·다른 헤더에는 넣지 않습니다.
        // 헤더 값은 HTTP 라이브러리 디버그·추적 로그에서도 가립니다.
        if let Some(api_key) = &self.api_key {
            let mut value =
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key.reveal()))
                    .map_err(SendFailure::build_error)?;
            // 라이브러리 내부 로그에서 키가 드러나지 않게 민감 헤더로 표시합니다(보안 기본).
            value.set_sensitive(true);
            builder = builder.header(reqwest::header::AUTHORIZATION, value);
        }

        let response = self
            .wait_for_headers(builder, cancel)
            .await
            .map_err(SendFailure::from)?;
        if !response.status().is_success() {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned);
            return Err(SendFailure {
                error: self.server_status_error(response, cancel).await,
                retry_after,
            });
        }
        Ok(response)
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
            // 판단을 거치지 않은 단일 요청 실패에는 포기 이유가 없다.
            give_up: None,
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

/// 재시도 판단에 넘기는 전송 실패 봉지입니다.
///
/// 방금 실패한 오류와, 응답이 준 `Retry-After` 헤더 원문 값만 담습니다.
/// 헤더 원문은 판단이 소비하고 공개 오류에는 남지 않습니다(02-6 spec 33행).
pub(super) struct SendFailure {
    /// 방금 실패한 전송 오류입니다.
    pub(super) error: TransportError,
    /// `Retry-After` 헤더 원문입니다. 헤더가 없거나 읽을 수 없으면 `None`입니다.
    pub(super) retry_after: Option<String>,
}

impl SendFailure {
    /// 헤더 값 조립이 실패한 봉지를 만듭니다. 키 값이 들어간 원문은 버리고 종류만 남깁니다.
    fn build_error(_error: reqwest::header::InvalidHeaderValue) -> Self {
        Self {
            error: TransportError::ClientBuild,
            retry_after: None,
        }
    }
}

/// 헤더를 받기 전 단계 실패 봉지입니다. 응답이 없으니 헤더 값도 없습니다.
impl From<TransportError> for SendFailure {
    fn from(error: TransportError) -> Self {
        Self {
            error,
            retry_after: None,
        }
    }
}
