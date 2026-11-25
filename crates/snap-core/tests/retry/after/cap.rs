//! 상한 판별 시험. spec AC 1의 상한 이하·초과 경계를 초·HTTP 날짜 둘 다로 확인함.

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use snap_core::llm::RetryAfterOutcome;
use snap_core::llm::interpret_retry_after;

/// 1970-01-01 00:00:00 UTC.
const EPOCH: SystemTime = UNIX_EPOCH;

/// 상한. 초·HTTP 날짜 둘 다의 상한 경계를 보는 값.
pub(super) const CAP: Duration = Duration::from_secs(600);

/// 상한이 0초인 설정. 모든 대기가 초과.
const ZERO_CAP: Duration = Duration::ZERO;

/// 상한이 가장 큰 설정. 초과 미발생.
const MAX_CAP: Duration = Duration::from_secs(u64::MAX);

/// 상한과 같은 초는 대기.
#[test]
fn cap_boundary_seconds_is_wait() {
    let outcome = interpret_retry_after(Some("60"), EPOCH, Duration::from_secs(60));
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::from_secs(60)));
}

/// 상한보다 1초 큰 초는 대기 초를 밝힌 포기.
#[test]
fn one_second_over_cap_is_exceeds_cap() {
    let outcome = interpret_retry_after(Some("61"), EPOCH, Duration::from_secs(60));
    assert_eq!(
        outcome,
        RetryAfterOutcome::ExceedsCap {
            wait: Duration::from_secs(61)
        }
    );
}

/// 상한이 0이어도 0초는 대기.
#[test]
fn zero_cap_with_zero_wait_is_wait() {
    let outcome = interpret_retry_after(Some("0"), EPOCH, ZERO_CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::ZERO));
}

/// 상한이 0이면 1초는 초과.
#[test]
fn zero_cap_with_one_second_is_exceeds_cap() {
    let outcome = interpret_retry_after(Some("1"), EPOCH, ZERO_CAP);
    assert_eq!(
        outcome,
        RetryAfterOutcome::ExceedsCap {
            wait: Duration::from_secs(1)
        }
    );
}

/// 상한이 가장 크면 큰 초도 대기.
#[test]
fn max_cap_with_large_seconds_is_wait() {
    let outcome = interpret_retry_after(Some("999999999999"), EPOCH, MAX_CAP);
    assert_eq!(
        outcome,
        RetryAfterOutcome::Wait(Duration::from_secs(999_999_999_999))
    );
}

/// 상한과 같은 초는 대기.
#[test]
fn cap_seconds_is_wait() {
    let outcome = interpret_retry_after(Some("600"), EPOCH, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::from_secs(600)));
}

/// 상한과 같은 HTTP 날짜는 대기.
#[test]
fn cap_date_is_wait() {
    let outcome = interpret_retry_after(Some("Thu, 01 Jan 1970 00:10:00 GMT"), EPOCH, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::from_secs(600)));
}

/// 상한을 넘는 HTTP 날짜는 대기 초를 밝힌 포기.
#[test]
fn over_cap_date_is_exceeds_cap() {
    let outcome = interpret_retry_after(Some("Thu, 01 Jan 1970 00:10:01 GMT"), EPOCH, CAP);
    assert_eq!(
        outcome,
        RetryAfterOutcome::ExceedsCap {
            wait: Duration::from_secs(601)
        }
    );
}
