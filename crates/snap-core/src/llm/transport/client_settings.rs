//! 클라이언트가 가진 설정 값·주입 값을 읽거나 올리는 접근자들입니다(02-6·02-7).
//!
//! 생성·구조 관리(`client.rs`)와 이유가 다른 파일로 나눴다(SRP, dev-principles 2절).
//! 재시도 판단에 쓰는 값, 키·주소 가림 도구를 다룬다.

use crate::llm::error::scrub::SecretScrub;
use crate::llm::retry::RetrySettings;
use crate::llm::retry::retry_map::retry_settings;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 백오프 난수를 고정하는 시험 전용 값 올리기입니다.
    ///
    /// 제품 코드는 이 경로를 쓰지 않고 기본 `None`(생산 공급)입니다.
    /// 시험 지원(`tests/common/support.rs`)의 `set_unit_sample` 헬퍼 한 곳에서만 부릅니다.
    #[allow(dead_code, clippy::must_use_candidate)]
    pub fn set_unit_sample(mut self, unit_sample: f64) -> Self {
        self.unit_sample_override = Some(unit_sample);
        self
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
