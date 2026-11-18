//! Full Jitter 백오프와 시도 횟수 판정.
//!
//! 헤더 없음·연결 오류일 때 캡드 지수 상한 안 균등 난수로 대기. 횟수 초과가 상한 초과보다 우선.

use std::time::Duration;

/// 백오프 경로(헤더 없음·연결 오류) 판단 결과.
///
/// 포기 이유에는 시도 횟수만 담고 키·호스트·본문은 담지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BackoffDecision {
    /// Full Jitter로 이만큼 기다린 뒤 재시도.
    Wait(Duration),
    /// 최대 시도 횟수를 넘어 포기.
    AttemptsExceeded {
        /// 현재까지 센 시도 횟수(첫 요청=1).
        attempt: u32,
    },
}

/// 횟수 초과와 `Retry-After` 상한 초과 중 고른 포기 이유.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GiveUpPriority {
    /// 시도 횟수 초과. 상한 초과와 동시여도 이쪽.
    AttemptsExceeded {
        /// 현재까지 센 시도 횟수(첫 요청=1).
        attempt: u32,
    },
    /// 횟수는 남았고 `Retry-After`만 상한을 넘음.
    RetryAfterExceedsCap,
}

/// 캡드 지수 상한: `min(cap, base × 2^(attempt-1))`.
///
/// `attempt`는 첫 요청=1. 0이면 `2^0`. 지수 곱이 커져도 `cap`으로 막아 오버플로를 피함.
pub(crate) fn capped_exponential_ceiling(attempt: u32, base: Duration, cap: Duration) -> Duration {
    if base.is_zero() || cap.is_zero() {
        return Duration::ZERO;
    }
    if base >= cap {
        return cap;
    }

    let exp = attempt.saturating_sub(1);
    let mut ceiling = base;
    for _ in 0..exp {
        ceiling = ceiling.saturating_mul(2);
        if ceiling >= cap {
            return cap;
        }
    }
    ceiling
}

/// Full Jitter 대기: `[0, ceiling]`에서 `unit_sample`(0.0..=1.0)로 고름.
///
/// 난수는 호출측이 주입. `0.0`이면 0, `1.0`이면 상한.
pub(crate) fn full_jitter_wait(ceiling: Duration, unit_sample: f64) -> Duration {
    if ceiling.is_zero() {
        return Duration::ZERO;
    }
    let t = clamp_unit(unit_sample);
    if t <= 0.0 {
        return Duration::ZERO;
    }
    if t >= 1.0 {
        return ceiling;
    }
    let nanos = (ceiling.as_nanos() as f64) * t;
    Duration::from_nanos(nanos as u64)
}

/// 시도 횟수가 최대 재시도를 넘었는지.
///
/// `attempt`는 방금 실패한 요청의 시도 번호(다음 시도를 사전 게이트하지 않음).
/// 첫 요청=1·`retries=3`이면 요청 최대 4. `attempt > retries`이면 초과.
pub(crate) fn is_attempts_exceeded(attempt: u32, retries: u32) -> bool {
    attempt > retries
}

/// 헤더 없음 경로: 횟수를 먼저 보고, 남으면 Full Jitter 대기를 돌림.
///
/// `attempt`는 방금 실패한 요청의 시도 번호. `base`/`cap`/`retries`는 `Duration`·`u32`만 받음.
pub(crate) fn decide_backoff(
    attempt: u32,
    retries: u32,
    base: Duration,
    cap: Duration,
    unit_sample: f64,
) -> BackoffDecision {
    if is_attempts_exceeded(attempt, retries) {
        return BackoffDecision::AttemptsExceeded { attempt };
    }
    let ceiling = capped_exponential_ceiling(attempt, base, cap);
    BackoffDecision::Wait(full_jitter_wait(ceiling, unit_sample))
}

/// 횟수 초과와 `Retry-After` 상한 초과 신호 중 포기 이유를 고름.
///
/// 상한 초과는 이미 계산된 불리언만 받음. 둘 다면 [`GiveUpPriority::AttemptsExceeded`].
pub(crate) fn resolve_give_up_priority(
    attempt: u32,
    retries: u32,
    retry_after_exceeds_cap: bool,
) -> Option<GiveUpPriority> {
    if is_attempts_exceeded(attempt, retries) {
        return Some(GiveUpPriority::AttemptsExceeded { attempt });
    }
    if retry_after_exceeds_cap {
        return Some(GiveUpPriority::RetryAfterExceedsCap);
    }
    None
}

fn clamp_unit(sample: f64) -> f64 {
    if sample.is_nan() {
        return 0.0;
    }
    sample.clamp(0.0, 1.0)
}
