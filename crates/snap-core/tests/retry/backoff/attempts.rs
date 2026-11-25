//! 시도 횟수 초과 판별 시험. spec AC 3의 경계(경계 통과·초과)를 확인함.

use snap_core::llm::is_attempt_limit_exceeded;

/// 최대 재시도 3회면 첫 요청(시도 1)부터 시도 4까지 통과.
#[test]
fn attempts_up_to_max_plus_one_pass() {
    for attempt in 1_u64..=4 {
        assert!(!is_attempt_limit_exceeded(attempt, 3));
    }
}

/// 최대 재시도 3회에서 시도 5는 초과.
#[test]
fn attempt_over_max_plus_one_is_exceeded() {
    assert!(is_attempt_limit_exceeded(5, 3));
    assert!(is_attempt_limit_exceeded(6, 3));
}

/// 최대 재시도 0회면 시도 1은 통과, 시도 2는 초과.
#[test]
fn zero_max_retries_boundary() {
    assert!(!is_attempt_limit_exceeded(1, 0));
    assert!(is_attempt_limit_exceeded(2, 0));
}

/// 첫 요청인 시도 0은 최대 재시도 횟수와 무관하게 통과.
#[test]
fn first_attempt_never_exceeded() {
    assert!(!is_attempt_limit_exceeded(0, 0));
    assert!(!is_attempt_limit_exceeded(0, 3));
}

/// 최대 재시도 최대로도 시도 수는 초과로 판별.
#[test]
fn max_retries_ceiling_does_not_overflow() {
    assert!(!is_attempt_limit_exceeded(u64::MAX, u64::MAX));
    assert!(is_attempt_limit_exceeded(u64::MAX, u64::MAX - 2));
}
