//! 클라이언트가 가진 설정 값·주입 값을 읽거나 올리는 접근자들입니다(02-6·02-7).
//!
//! 생성·구조 관리(`client.rs`)와 이유가 다른 파일로 나눴다(SRP, dev-principles 2절).
//! 재시도 판단에 쓰는 값, 키·주소 가림 도구, 시험용 백오프 난수 주입을 다룬다.

use crate::llm::error::scrub::SecretScrub;
use crate::llm::retry::RetrySettings;
use crate::llm::retry::retry_map::retry_settings;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
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
