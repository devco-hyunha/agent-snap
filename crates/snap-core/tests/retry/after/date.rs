//! HTTP 날짜(IMF-fixdate) 해석 시험. spec AC 1·2의 정상·과거·미래·해석 불가 확인.
//!
//! 현재 시각은 주입하고, 실제 대기는 하지 않음.

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use snap_core::llm::RetryAfterOutcome;
use snap_core::llm::interpret_retry_after;

use super::cap::CAP;

/// 1970-01-01 00:00:00 UTC.
const EPOCH: SystemTime = UNIX_EPOCH;

/// 1970-01-01 기준 초만큼 지난 시각.
fn epoch_after(seconds: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(seconds)
}

/// 2024-02-28 00:00:00 UTC.
fn leap_day_before() -> SystemTime {
    epoch_after(1_709_078_400)
}

/// 미래 HTTP 날짜는 그 초만큼 대기.
#[test]
fn future_date_becomes_wait() {
    let outcome = interpret_retry_after(Some("Thu, 01 Jan 1970 00:00:30 GMT"), EPOCH, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::from_secs(30)));
}

/// 과거 HTTP 날짜는 대기 0초.
#[test]
fn past_date_becomes_wait_zero() {
    let outcome = interpret_retry_after(Some("Wed, 31 Dec 1969 23:59:59 GMT"), EPOCH, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::ZERO));
}

/// 현재 시각과 같은 HTTP 날짜는 대기 0초.
#[test]
fn same_date_becomes_wait_zero() {
    let outcome = interpret_retry_after(Some("Thu, 01 Jan 1970 00:00:00 GMT"), EPOCH, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::ZERO));
}

/// 윤년 2월 29일 읽기. 하루 전 시각 기준 하루 대기는 상한 초과.
#[test]
fn leap_day_date_is_read_as_one_day() {
    let outcome = interpret_retry_after(
        Some("Thu, 29 Feb 2024 00:00:00 GMT"),
        leap_day_before(),
        CAP,
    );
    assert_eq!(
        outcome,
        RetryAfterOutcome::ExceedsCap {
            wait: Duration::from_secs(86_400)
        }
    );
}

/// 윤초(60초)를 허용.
#[test]
fn leap_second_is_accepted() {
    let outcome = interpret_retry_after(Some("Thu, 01 Jan 1970 00:00:60 GMT"), EPOCH, CAP);
    assert_eq!(outcome, RetryAfterOutcome::Wait(Duration::from_secs(60)));
}

/// 형식 오류·존 이름 오류는 해석 불가로 헤더 없음.
#[test]
fn malformed_date_is_absent() {
    let headers = [
        Some("Thu 01 Jan 1970 00:00:00 GMT"),
        Some("Xxx, 01 Jan 1970 00:00:00 GMT"),
        Some("Thu, 01 Xan 1970 00:00:00 GMT"),
        Some("Thu, 1 Jan 1970 00:00:00 GMT"),
        Some("Thu, 01 Jan 197 00:00:00 GMT"),
        Some("Thu, 01 Jan 1970 00:00:00 UTC"),
        Some("Thu, 01 Jan 1970 00:00:00"),
        Some("Thu, 01 Jan 1970 24:00:00 GMT"),
        Some("Thu, 01 Jan 1970 00:60:00 GMT"),
        Some("Thu, 01 Jan 1970 00:00:00 GMT extra"),
    ];
    for header in headers {
        let outcome = interpret_retry_after(header, EPOCH, CAP);
        assert_eq!(outcome, RetryAfterOutcome::Absent, "header {header:?}");
    }
}

/// 달력에 없는 날은 해석 불가.
#[test]
fn impossible_calendar_day_is_absent() {
    let headers = [
        Some("Fri, 30 Feb 2024 00:00:00 GMT"),
        Some("Fri, 29 Feb 2023 00:00:00 GMT"),
        Some("Fri, 00 Dec 2024 00:00:00 GMT"),
        Some("Fri, 32 Dec 2024 00:00:00 GMT"),
        Some("Fri, 31 Nov 2024 00:00:00 GMT"),
    ];
    for header in headers {
        let outcome = interpret_retry_after(header, EPOCH, CAP);
        assert_eq!(outcome, RetryAfterOutcome::Absent, "header {header:?}");
    }
}
