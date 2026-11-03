//! `Retry-After` 해석·상한 비교 단위 테스트(AC 1~3, spec AC 1·2).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::llm::retry::after::{RetryAfterDecision, interpret_retry_after};

const CAP_60: Duration = Duration::from_secs(60);

fn fixed_now() -> SystemTime {
    // 2015-10-21 07:28:00 GMT
    UNIX_EPOCH + Duration::from_secs(1_445_412_480)
}

fn decide(header: Option<&str>) -> RetryAfterDecision {
    interpret_retry_after(header, fixed_now(), CAP_60)
}

#[test]
fn delay_seconds_within_cap_waits() {
    assert_eq!(
        decide(Some("30")),
        RetryAfterDecision::Wait(Duration::from_secs(30))
    );
}

#[test]
fn delay_seconds_equal_cap_waits() {
    assert_eq!(
        decide(Some("60")),
        RetryAfterDecision::Wait(Duration::from_secs(60))
    );
}

#[test]
fn delay_seconds_above_cap_exceeds() {
    assert_eq!(
        decide(Some("61")),
        RetryAfterDecision::ExceedsCap {
            wait: Duration::from_secs(61)
        }
    );
}

#[test]
fn http_date_within_cap_waits() {
    // now + 45s
    let header = "Wed, 21 Oct 2015 07:28:45 GMT";
    assert_eq!(
        decide(Some(header)),
        RetryAfterDecision::Wait(Duration::from_secs(45))
    );
}

#[test]
fn http_date_equal_cap_waits() {
    // now + 60s (== cap)
    let header = "Wed, 21 Oct 2015 07:29:00 GMT";
    assert_eq!(
        decide(Some(header)),
        RetryAfterDecision::Wait(Duration::from_secs(60))
    );
}

#[test]
fn http_date_above_cap_exceeds() {
    // now + 120s
    let header = "Wed, 21 Oct 2015 07:30:00 GMT";
    assert_eq!(
        decide(Some(header)),
        RetryAfterDecision::ExceedsCap {
            wait: Duration::from_secs(120)
        }
    );
}

#[test]
fn zero_delay_waits_zero() {
    assert_eq!(decide(Some("0")), RetryAfterDecision::Wait(Duration::ZERO));
}

#[test]
fn past_http_date_waits_zero() {
    let header = "Wed, 21 Oct 2015 07:27:00 GMT";
    assert_eq!(
        decide(Some(header)),
        RetryAfterDecision::Wait(Duration::ZERO)
    );
}

#[test]
fn pre_unix_epoch_http_date_waits_zero() {
    // 1969-12-31 23:59:59 GMT (Unix epoch 이전, AC2/D14)
    let header = "Wed, 31 Dec 1969 23:59:59 GMT";
    assert_eq!(
        decide(Some(header)),
        RetryAfterDecision::Wait(Duration::ZERO)
    );
}

#[test]
fn negative_is_absent() {
    assert_eq!(decide(Some("-1")), RetryAfterDecision::Absent);
    assert_eq!(decide(Some("-30")), RetryAfterDecision::Absent);
}

#[test]
fn unparseable_is_absent() {
    assert_eq!(decide(None), RetryAfterDecision::Absent);
    assert_eq!(decide(Some("")), RetryAfterDecision::Absent);
    assert_eq!(decide(Some("   ")), RetryAfterDecision::Absent);
    assert_eq!(decide(Some("soon")), RetryAfterDecision::Absent);
    assert_eq!(decide(Some("1.5")), RetryAfterDecision::Absent);
    assert_eq!(decide(Some("+10")), RetryAfterDecision::Absent);
    assert_eq!(
        decide(Some("21 Oct 2015 07:28:00 GMT")),
        RetryAfterDecision::Absent
    );
}

#[test]
fn exceeds_cap_carries_only_wait_duration() {
    let decision = decide(Some("90"));
    match decision {
        RetryAfterDecision::ExceedsCap { wait } => {
            assert_eq!(wait, Duration::from_secs(90));
            let debug = format!("{decision:?}");
            assert!(!debug.to_lowercase().contains("key"));
            assert!(!debug.contains("http"));
            assert!(!debug.contains("body"));
        }
        other => panic!("expected ExceedsCap, got {other:?}"),
    }
}

#[test]
fn trims_whitespace_around_delay_seconds() {
    assert_eq!(
        decide(Some("  12  ")),
        RetryAfterDecision::Wait(Duration::from_secs(12))
    );
}
