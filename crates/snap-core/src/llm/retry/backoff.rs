//! Full Jitter 백오프와 시도 횟수 판정입니다.
//!
//! 헤더가 없거나 재시도 후보 연결 오류일 때 캡드 지수 상한 안에서 균등 난수로 대기를 정합니다(D13).
//! 시도 횟수 초과와 `Retry-After` 상한 초과가 겹치면 횟수 초과를 우선합니다(D26).
//! 실제 sleep은 02-6입니다. 공개 조립은 [`super::decide::decide_retry`]가 맡습니다.

use std::time::Duration;

/// 백오프 경로(헤더 없음·연결 오류) 판단 결과입니다.
///
/// 포기 이유에는 시도 횟수만 담으며 키·호스트·응답 본문은 담지 않습니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BackoffDecision {
    /// Full Jitter로 이만큼 기다린 뒤 재시도합니다.
    Wait(Duration),
    /// 최대 시도 횟수를 넘어 포기합니다.
    AttemptsExceeded {
        /// 현재까지 센 시도 횟수입니다(첫 요청=1).
        attempt: u32,
    },
}

/// 횟수 초과와 `Retry-After` 상한 초과 중 고른 포기 이유입니다(D26).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GiveUpPriority {
    /// 시도 횟수 초과입니다. 상한 초과와 동시에 해당해도 이쪽입니다.
    AttemptsExceeded {
        /// 현재까지 센 시도 횟수입니다(첫 요청=1).
        attempt: u32,
    },
    /// 횟수는 남았고 `Retry-After`만 상한을 넘었습니다.
    RetryAfterExceedsCap,
}

/// 캡드 지수 상한: `min(cap, base × 2^(attempt-1))`입니다.
///
/// `attempt`는 첫 요청을 1로 셉니다. 0이면 첫 시도와 같이 `2^0`을 씁니다.
/// 지수 곱이 커져도 `cap`으로 먼저 막아 오버플로를 피합니다.
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

/// Full Jitter 대기: `[0, ceiling]`에서 `unit_sample`(0.0..=1.0)로 고릅니다.
///
/// 난수는 호출측이 주입합니다. `0.0`이면 0, `1.0`이면 상한입니다.
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

/// 시도 횟수가 최대 재시도를 넘었는지입니다.
///
/// `attempt`에는 **방금 실패한 요청의 시도 번호**를 넘깁니다(다음 시도를 사전 게이트하지 않음).
/// 첫 요청=1이고 `retries=3`이면 `attempt` 1..=4 실패 후 판정 시 요청 최대 4입니다(D14).
/// `attempt > retries`이면 초과입니다.
pub(crate) fn is_attempts_exceeded(attempt: u32, retries: u32) -> bool {
    attempt > retries
}

/// 헤더 없음 경로: 횟수를 먼저 보고, 남으면 Full Jitter 대기를 돌립니다.
///
/// `attempt`에는 **방금 실패한 요청의 시도 번호**를 넘깁니다(다음 시도를 사전 게이트하지 않음).
/// `retries=3`이면 `attempt` 1..=4 실패 후 판정 시 요청 최대 4입니다(D14).
/// `base`/`cap`/`retries`는 `Duration`·`u32`로만 받으며 `HttpSettings`에 의존하지 않습니다.
/// `unit_sample`은 `[0.0, 1.0]` 난수 샘플입니다.
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

/// 횟수 초과와 `Retry-After` 상한 초과 신호 중 포기 이유를 고릅니다(D26).
///
/// 상한 초과는 이미 계산된 불리언만 받습니다(헤더 문자열 파싱 없음).
/// 둘 다 해당하면 [`GiveUpPriority::AttemptsExceeded`]입니다.
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
