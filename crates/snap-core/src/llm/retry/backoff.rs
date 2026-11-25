//! 재시도 횟수·백오프 계산 자리. 실제 잠자기는 이 파일이 아니라 전송 쪽 소관.
//!
//! 상한은 `backoff_min × 2^(시도-1)`과 `backoff_max` 중 작은 값이고, 대기는 그 상한 안에서 균등 무작위로 정함.
//! 첫 요청은 시도 1로 셈. 최대 재시도 3회면 요청은 최대 4번이고, 시도 5부터 초과로 봄.

use std::time::Duration;

use crate::llm::retry::random::RandomSource;
use crate::llm::retry::settings::RetrySettings;

/// 시도마다 기본값에 곱하는 배율.
const BACKOFF_MULTIPLIER: u64 = 2;

/// 상한 계산: `backoff_min × 2^(시도-1)`과 `backoff_max` 중 작은 값.
///
/// 시도 1은 기본값 그대로. 곱셈은 포화 연산이라 넘침 없음.
pub fn backoff_cap(attempt: u64, settings: &RetrySettings) -> Duration {
    let scaled = scaled_backoff(attempt, settings.backoff_min);
    scaled.min(settings.backoff_max)
}

/// Full Jitter 대기: 0 이상 상한 이하의 균등 무작위.
///
/// AWS Full Jitter 공식 `random(0, min(cap, base × 2^attempt))`을 시도 1 기준으로 따름.
pub fn full_jitter_wait(
    attempt: u64,
    settings: &RetrySettings,
    rng: &mut dyn RandomSource,
) -> Duration {
    let cap = backoff_cap(attempt, settings);
    let max_nanos = nanos_to_u64(cap.as_nanos());
    let nanos = rng.next_inclusive(max_nanos);
    Duration::from_nanos(nanos)
}

/// 시도 횟수가 최대 재시도 횟수를 넘었는지 판별.
///
/// 첫 요청은 시도 1. 최대 재시도 `max_retries`면 시도 `max_retries + 1`까지 통과.
pub fn is_attempt_limit_exceeded(attempt: u64, max_retries: u64) -> bool {
    let last_allowed_attempt = max_retries.saturating_add(1);
    attempt > last_allowed_attempt
}

/// `backoff_min × 2^(시도-1)`을 나노초로 계산. 곱셈은 포화 연산.
fn scaled_backoff(attempt: u64, base: Duration) -> Duration {
    let exponent = u32::try_from(attempt.saturating_sub(1)).unwrap_or(u32::MAX);
    let factor = u128::from(BACKOFF_MULTIPLIER.saturating_pow(exponent));
    let nanos = base.as_nanos().saturating_mul(factor);
    Duration::from_nanos(nanos_to_u64(nanos))
}

/// `u128` 나노초를 `u64`로 조임. `u64`보다 크면 `u64` 최대로 봄.
fn nanos_to_u64(nanos: u128) -> u64 {
    u64::try_from(nanos).unwrap_or(u64::MAX)
}
