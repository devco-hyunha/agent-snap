//! OpenAI 호환 `/chat/completions` 스트리밍 클라이언트의 구조·생성.
//!
//! 주소·종류·키·시간 설정을 담고 클라이언트를 만드는 일만 함.

use std::sync::Arc;

use crate::config::{EndpointKind, HttpSettings};
use crate::llm::client::TransportError;
use crate::secrets::SecretValue;

/// OpenAI 호환 `/chat/completions` 스트리밍 클라이언트.
///
/// 엔드포인트 주소·종류·키·시간 설정을 갖고, 요청마다 본문만 받음.
pub struct OpenAiCompatClient {
    // 값 읽기는 transport 안 책임 파일들이 함께 쓰므로 `pub(super)`.
    pub(super) base_url: SecretValue,
    pub(super) kind: EndpointKind,
    pub(super) api_key: Option<SecretValue>,
    pub(super) http: HttpSettings,
    /// HTTP 전송용 클라이언트. 공개 타입으로 내보내지 않음.
    pub(super) http_client: Arc<reqwest::Client>,
    /// 백오프 난수 고정 주입값. 제품은 항상 `None`.
    pub(super) unit_sample_override: Option<f64>,
}

impl OpenAiCompatClient {
    /// 엔드포인트 정보와 시간 설정으로 클라이언트를 만듦.
    ///
    /// 연결 시간 초과는 HTTP 라이브러리에 맡김. 헤더 대기·유휴는 여기서 따로 잼.
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
            unit_sample_override: None,
        })
    }
}
