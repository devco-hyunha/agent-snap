//! delay-seconds(초 형식) 해석 시험. spec AC 2의 0·음수·해석 불가 경계 확인.

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use snap_core::llm::RetryAfterOutcome;
use snap_core::llm::interpret_retry_after;

/// 기준 시각. 1970-01-01 00:00:00 UTC.
const NOW: SystemTime = UNIX_EPOCH;

/// 상한은 `cap.rs` 정본을 재사용함.
use super::cap::CAP;

/// 초 형식 정수를 대기 시간으로 바꿈.
#[test]
fn delay_seconds_becomes_wait() {
    let outcome = interpret_retry_after(Some("30"), NOW, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::from_secs(30)));
}

/// 0초는 기다리지 않고 바로 재시도하는 대기 0초.
#[test]
fn zero_seconds_is_wait_zero() {
    let outcome = interpret_retry_after(Some("0"), NOW, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::ZERO));
}

/// 앞뒤 공백을 없앤 값도 읽음.
#[test]
fn delay_seconds_ignores_spaces() {
    let outcome = interpret_retry_after(Some("  7  "), NOW, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::from_secs(7)));
}

/// 음수는 해석할 수 없는 값이라 헤더 없음.
#[test]
fn negative_is_absent() {
    let outcome = interpret_retry_after(Some("-1"), NOW, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Absent);
}

/// 숫자 외·소수점·빈 문자열·빈 헤더는 모두 헤더 없음.
#[test]
fn unparseable_is_absent() {
    let values = [Some("abc"), Some("3.5"), Some(""), Some("   "), None];
    for header in values {
        let outcome = interpret_retry_after(header, NOW, CAP);
        assert_eq!(outcome, RetryAfterOutcome::Absent, "header {header:?}");
    }
}

/// 숫자 범위를 넘는 값은 해석 불가로 헤더 없음.
#[test]
fn too_big_seconds_is_absent() {
    let outcome = interpret_retry_after(Some("99999999999999999999999"), NOW, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Absent);
}
