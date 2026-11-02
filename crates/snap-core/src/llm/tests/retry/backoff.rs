//! Full Jitter·시도 횟수 단위 테스트(AC 1~3, spec AC 3·4).

use std::time::Duration;

use crate::llm::{
    BackoffDecision, GiveUpPriority, capped_exponential_ceiling, decide_backoff, full_jitter_wait,
    is_attempts_exceeded, resolve_give_up_priority,
};

const BASE_1S: Duration = Duration::from_secs(1);
const CAP_30S: Duration = Duration::from_secs(30);

#[test]
fn ceiling_grows_by_powers_of_two_until_cap() {
    assert_eq!(
        capped_exponential_ceiling(1, BASE_1S, CAP_30S),
        Duration::from_secs(1)
    );
    assert_eq!(
        capped_exponential_ceiling(2, BASE_1S, CAP_30S),
        Duration::from_secs(2)
    );
    assert_eq!(
        capped_exponential_ceiling(3, BASE_1S, CAP_30S),
        Duration::from_secs(4)
    );
    assert_eq!(
        capped_exponential_ceiling(4, BASE_1S, CAP_30S),
        Duration::from_secs(8)
    );
    assert_eq!(
        capped_exponential_ceiling(5, BASE_1S, CAP_30S),
        Duration::from_secs(16)
    );
    // 1 × 2^5 = 32 → cap 30
    assert_eq!(
        capped_exponential_ceiling(6, BASE_1S, CAP_30S),
        Duration::from_secs(30)
    );
    assert_eq!(
        capped_exponential_ceiling(20, BASE_1S, CAP_30S),
        Duration::from_secs(30)
    );
}

#[test]
fn full_jitter_zero_and_ceiling() {
    let ceiling = Duration::from_secs(8);
    assert_eq!(full_jitter_wait(ceiling, 0.0), Duration::ZERO);
    assert_eq!(full_jitter_wait(ceiling, 1.0), ceiling);
}

#[test]
fn decide_backoff_respects_ceiling_with_fixed_samples() {
    // 시도 1: ceiling = 1s
    assert_eq!(
        decide_backoff(1, 3, BASE_1S, CAP_30S, 0.0),
        BackoffDecision::Wait(Duration::ZERO)
    );
    assert_eq!(
        decide_backoff(1, 3, BASE_1S, CAP_30S, 1.0),
        BackoffDecision::Wait(Duration::from_secs(1))
    );
    // 시도 2: ceiling = 2s
    assert_eq!(
        decide_backoff(2, 3, BASE_1S, CAP_30S, 1.0),
        BackoffDecision::Wait(Duration::from_secs(2))
    );
    // 시도 6: ceiling = cap 30s
    assert_eq!(
        decide_backoff(6, 10, BASE_1S, CAP_30S, 1.0),
        BackoffDecision::Wait(Duration::from_secs(30))
    );
}

#[test]
fn attempts_exceeded_when_past_retries() {
    // retries=3 → 요청 최대 4번. 시도 1~3 실패 후에는 재시도, 4번째 실패 후 포기(D14).
    assert!(!is_attempts_exceeded(1, 3));
    assert!(!is_attempts_exceeded(3, 3));
    assert!(is_attempts_exceeded(4, 3));
    assert!(is_attempts_exceeded(5, 3));
    assert!(is_attempts_exceeded(1, 0));
}

#[test]
fn decide_backoff_gives_up_when_attempts_exceeded() {
    assert_eq!(
        decide_backoff(4, 3, BASE_1S, CAP_30S, 0.5),
        BackoffDecision::AttemptsExceeded { attempt: 4 }
    );
}

#[test]
fn give_up_priority_prefers_attempts_over_retry_after_cap() {
    assert_eq!(
        resolve_give_up_priority(4, 3, true),
        Some(GiveUpPriority::AttemptsExceeded { attempt: 4 })
    );
    assert_eq!(
        resolve_give_up_priority(4, 3, false),
        Some(GiveUpPriority::AttemptsExceeded { attempt: 4 })
    );
    assert_eq!(
        resolve_give_up_priority(2, 3, true),
        Some(GiveUpPriority::RetryAfterExceedsCap)
    );
    assert_eq!(resolve_give_up_priority(2, 3, false), None);
}
