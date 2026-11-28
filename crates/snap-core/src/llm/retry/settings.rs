//! 재시도 설정값 타입 자리. `Duration`·정수 값만 받고 설정 타입에는 의존하지 않음.

use std::time::Duration;

/// 재시도 판단이 받는 설정값.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RetrySettings {
    /// 최대 재시도 횟수. 첫 요청은 시도 횟수 1로 셈.
    pub max_retries: u64,

    /// 백오프 기본값.
    pub backoff_min: Duration,

    /// 백오프 상한.
    pub backoff_max: Duration,

    /// `Retry-After`가 받을 수 있는 최대 초.
    pub retry_after_max: u64,
}
