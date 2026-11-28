//! 백오프 상한 계산 시험. spec AC 3의 `기본값 × 2^(시도-1)`과 상한 중 작은 값을 확인함.

use std::time::Duration;

use snap_core::llm::RetrySettings;
use snap_core::llm::backoff_cap;

use crate::fixtures::SETTINGS;

/// 기본값이 0초인 설정. 상한이 0이어야 함.
const ZERO_BASE: RetrySettings = RetrySettings {
    max_retries: 3,
    backoff_min: Duration::ZERO,
    backoff_max: Duration::from_secs(30),
    retry_after_max: 60,
};

/// 기본값이 상한보다 큰 설정. 시도 1부터 상한과 같아야 함.
const OVER_BASE: RetrySettings = RetrySettings {
    max_retries: 3,
    backoff_min: Duration::from_secs(120),
    backoff_max: Duration::from_secs(30),
    retry_after_max: 60,
};

/// 시도 1은 기본값과 같은 상한.
#[test]
fn first_attempt_cap_is_base() {
    assert_eq!(backoff_cap(1, &SETTINGS), Duration::from_secs(1));
}

/// 시도 2·3·4는 기본값에 2를 곱한 배수.
#[test]
fn doubled_cap_per_attempt() {
    assert_eq!(backoff_cap(2, &SETTINGS), Duration::from_secs(2));
    assert_eq!(backoff_cap(3, &SETTINGS), Duration::from_secs(4));
    assert_eq!(backoff_cap(4, &SETTINGS), Duration::from_secs(8));
}

/// 배수가 상한을 넘기면 상한과 같음.
#[test]
fn cap_stops_at_backoff_max() {
    assert_eq!(backoff_cap(6, &SETTINGS), Duration::from_secs(30));
    assert_eq!(backoff_cap(64, &SETTINGS), Duration::from_secs(30));
}

/// 기본값이 0이면 상한도 0.
#[test]
fn zero_base_cap_is_zero() {
    assert_eq!(backoff_cap(1, &ZERO_BASE), Duration::ZERO);
    assert_eq!(backoff_cap(8, &ZERO_BASE), Duration::ZERO);
}

/// 기본값이 상한보다 크면 시도 1부터 상한.
#[test]
fn base_over_cap_is_clamped_to_cap() {
    assert_eq!(backoff_cap(1, &OVER_BASE), Duration::from_secs(30));
}

/// 시도 수가 매우 커도 지수 곱셈이 넘치지 않음.
#[test]
fn huge_attempt_does_not_overflow() {
    assert_eq!(backoff_cap(u64::MAX, &SETTINGS), Duration::from_secs(30));
    assert_eq!(backoff_cap(u64::MAX, &ZERO_BASE), Duration::ZERO);
}
