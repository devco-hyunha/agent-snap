//! OpenAI 호환 스트리밍 전송 구현 자리. 요청 보내기·조각 읽기·취소 연결 정리만 담당.
//!
//! reqwest 타입은 이 파일 안에서만 사용. 트레이트 서명에는 미포함.

use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::config::secret_string::SecretString;
use crate::config::types::HttpSettings;
use crate::llm::ChatRequest;
use crate::llm::EndpointKind;
use crate::llm::LlmClient;
use crate::llm::RestoreReceiver;
use crate::llm::TransportError;
use crate::llm::client::body::build_body;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::client::open::open_stream;
use crate::llm::client::read::read_stream;
use crate::llm::client::redact::hide_stream;
use crate::llm::client::timings::TransportTimings;

/// OpenAI 호환 엔드포인트 하나에 대한 스트리밍 전송 구현.
///
/// 연결 시간은 reqwest 연결 시간 설정에만 맡기고, 헤더 대기와 스트림 유휴는 따로 잰다.
#[derive(Debug, Clone)]
pub struct HttpTransport {
    /// 검증이 통과한 엔드포인트 주소. `/v1`까지 포함.
    base_url: SecretString,

    /// 요청 변환이 쓰는 서버 형식.
    kind: EndpointKind,

    /// 자격 파일에서 정한 키. 없으면 인증 헤더를 보내지 않음.
    key: Option<SecretString>,

    /// 연결·헤더 대기·스트림 유휴 시간.
    timings: TransportTimings,
}

impl HttpTransport {
    /// 검증이 통과한 엔드포인트 정보로 전송 하나 만들기.
    pub fn new(
        base_url: &str,
        kind: EndpointKind,
        key: Option<SecretString>,
        settings: &HttpSettings,
    ) -> Self {
        Self::with_timings(
            base_url,
            kind,
            key,
            TransportTimings::from_settings(settings),
        )
    }

    /// 지정한 시간 값으로 전송 만들기. 시험용 짧은 값 주입 창구.
    pub fn with_timings(
        base_url: &str,
        kind: EndpointKind,
        key: Option<SecretString>,
        timings: TransportTimings,
    ) -> Self {
        Self {
            base_url: SecretString::new(base_url.to_owned()),
            kind,
            key,
            timings,
        }
    }

    /// 이 요청이 알고 있는 키·엔드포인트 주소. 2차 가리기 대상으로만 씀.
    fn secrets(&self) -> KnownSecrets {
        KnownSecrets::new(self.key.clone(), self.base_url.expose())
    }

    /// 요청을 보내고 끝까지 읽은 수신자로 스트림을 여는 미래.
    async fn stream(
        self,
        body: Value,
        cancel: CancellationToken,
    ) -> Result<RestoreReceiver, TransportError> {
        let request_start = Instant::now();
        let secrets = self.secrets();
        let mut response = open_stream(
            self.base_url.expose(),
            self.key.as_ref(),
            &self.timings,
            &body,
            &cancel,
            &secrets,
        )
        .await?;
        let mut receiver = RestoreReceiver::new(request_start);
        let read = read_stream(&mut response, &mut receiver, &self.timings, &cancel).await;
        // 읽기가 끝나면 응답 값을 버려 연결을 정리한다. 취소·유휴도 같은 경로로 끝난다.
        drop(response);
        // 스트림 오류는 어림 가리기만 지난 요약이라 알고 있는 키·호스트로 한 번 더 가림.
        read.map_err(|error| hide_stream(error, &secrets))?;
        Ok(receiver)
    }
}

impl LlmClient for HttpTransport {
    type Open<'a> =
        Pin<Box<dyn Future<Output = Result<RestoreReceiver, TransportError>> + Send + 'a>>;
    type Receiver = RestoreReceiver;

    fn open(&self, request: &ChatRequest, cancel: CancellationToken) -> Self::Open<'_> {
        let body = build_body(request, self.kind);
        Box::pin(self.clone().stream(body, cancel))
    }
}
