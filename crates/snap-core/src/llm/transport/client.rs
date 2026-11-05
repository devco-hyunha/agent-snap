//! OpenAI 호환 `/chat/completions` 스트리밍 클라이언트의 구조·설정 관리입니다.
//!
//! 주소·종류·키·시간 설정을 담고 클라이언트를 만드는 일만 합니다. 전송 한 번은 `send_once`가
//! 재시도 고리는 `stream_chat`가 스트림 열기는 `open_stream`이 각각 맡습니다(작업 07 SRP 분리).

use std::sync::Arc;

use crate::config::{EndpointKind, HttpSettings};
use crate::llm::client::TransportError;
use crate::llm::error::scrub::SecretScrub;
use crate::llm::retry::RetrySettings;
use crate::llm::retry::retry_map::retry_settings;
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

    /// 백오프 난수를 고정하는 시험 전용 주입 경로입니다.
    ///
    /// 제품 코드는 이 경로를 쓰지 않고 기본 `None`(생산 공급)입니다.
    /// 테스트가 대기 시간을 정해진 값으로 고정해 시험 간격을 일정하게 유지하려 값만 받아 올립니다
    /// (02-6 시험 계획의 시간 주입).
    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn with_unit_sample(mut self, unit_sample: f64) -> Self {
        self.set_unit_sample(unit_sample);
        self
    }

    /// 바깥 시험(개발 의존으로 붙은 같은 크레이트 빌드)이 이 값을 붙일 수 있게 여는 길입니다.
    ///
    /// 라이브러리 안 시험과 제품 코드는 위 cfg(test) 경로를 그대로 쓰고,
    /// 이 경로는 개발 의존 빌드(cfg(test)가 켜지지 않는 자리)에서만 열립니다.
    /// 공개면 좁히기(D28)는 제품 코드와 라이브러리 안 시험 쪽만 적용하고,
    /// 개발 의존 빌드 경로는 시험 전용 자리이므로 공개로 열어 시험이 붙일 수 있게 합니다.
    #[cfg(not(test))]
    #[allow(dead_code, clippy::missing_errors_doc, clippy::must_use_candidate)]
    pub fn with_unit_sample(mut self, unit_sample: f64) -> Self {
        self.set_unit_sample(unit_sample);
        self
    }

    /// 백오프 난수 고정 주입 값을 자리에 올립니다. 값 두기만 한 자리라 두 경로가 같은 일을 합니다.
    fn set_unit_sample(&mut self, unit_sample: f64) {
        self.unit_sample_override = Some(unit_sample);
    }

    /// 이 요청에서 알고 있는 키·base_url로 가릴 가림 도구입니다(D28).
    pub(super) fn scrub(&self) -> SecretScrub {
        SecretScrub::new(self.api_key.as_ref(), &self.base_url)
    }

    /// 재시도 판단에 쓸 설정 값입니다. `http` 설정을 02-4 판단 값으로 옮깁니다(2단계 매핑 재사용).
    pub(super) fn retry_settings(&self) -> RetrySettings {
        retry_settings(&self.http)
    }

    /// 백오프·포기 판정에 쓰는 난수 값입니다. 제품은 `None`(기본)입니다.
    ///
    /// `None`이면 판단(`retry_loop`)이 생산 공급(`fastrand`)으로 채웁니다.
    pub(super) fn unit_sample(&self) -> Option<f64> {
        self.unit_sample_override
    }
}
