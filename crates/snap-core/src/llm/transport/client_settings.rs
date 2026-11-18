//! 클라이언트가 가진 설정 값·주입 값을 읽거나 올리는 접근자.
//!
//! 생성·구조(`client.rs`)와 이유를 나눔. 재시도 판단 값·키·주소 가림 도구를 다룸.

use crate::llm::error::scrub::SecretScrub;
use crate::llm::retry::RetrySettings;
use crate::llm::retry::retry_map::retry_settings;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 백오프 난수를 고정하는 시험 전용 값 올리기.
    ///
    /// 제품은 이 경로를 쓰지 않고 기본 `None`. 시험 지원 헬퍼 한 곳에서만 부름.
    #[allow(dead_code, clippy::must_use_candidate)]
    pub fn set_unit_sample(mut self, unit_sample: f64) -> Self {
        self.unit_sample_override = Some(unit_sample);
        self
    }

    /// 이 요청에서 알고 있는 키·base_url로 가릴 가림 도구.
    pub(super) fn scrub(&self) -> SecretScrub {
        SecretScrub::new(self.api_key.as_ref(), &self.base_url)
    }

    /// 재시도 판단에 쓸 설정 값. `http` 설정을 판단 값으로 옮김.
    pub(super) fn retry_settings(&self) -> RetrySettings {
        retry_settings(&self.http)
    }

    /// 백오프·포기 판정에 쓰는 난수 값. 제품은 `None`.
    ///
    /// `None`이면 판단이 생산 난수로 채움.
    pub(super) fn unit_sample(&self) -> Option<f64> {
        self.unit_sample_override
    }
}
