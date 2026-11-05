//! OpenAI 호환 `/chat/completions` 스트리밍 클라이언트의 구조·생성 책임입니다(02-5).
//!
//! 주소·종류·키·시간 설정을 담고 클라이언트를 만드는 일만 합니다. 설정 값 읽기·쓰기는
//! `client_settings`, 전송 한 번은 `send_once`, 재시도 고리는 `stream_chat`, 스트림 열기는
//! `open_stream`이 각각 맡습니다(작업 07 SRP 분리, 02-7 목록 조회 추가).

use std::sync::Arc;

use crate::config::{EndpointKind, HttpSettings};
use crate::llm::client::TransportError;
use crate::secrets::SecretValue;

/// OpenAI 호환 `/chat/completions` 스트리밍 클라이언트입니다.
///
/// 엔드포인트 주소·종류·키·시간 설정을 갖고, 요청마다 본문만 받습니다.
pub struct OpenAiCompatClient {
    // 값 읽기는 transport 안의 책임 파일들이 함께 쓰므로 `pub(super)`로 둡니다(작업 07 SRP 분리).
    pub(super) base_url: SecretValue,
    pub(super) kind: EndpointKind,
    pub(super) api_key: Option<SecretValue>,
    pub(super) http: HttpSettings,
    /// HTTP 전송용 클라이언트입니다. 공개 타입으로 내보내지 않습니다.
    pub(super) http_client: Arc<reqwest::Client>,
    /// 백오프 난수 고정 주입값입니다. 제품은 항상 `None`입니다.
    pub(super) unit_sample_override: Option<f64>,
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
            unit_sample_override: None,
        })
    }
}
