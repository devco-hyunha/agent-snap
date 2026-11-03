//! 공개 재시도 판단 진입점 단위 테스트(AC 1~4, spec AC 1~7 스모크).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::llm::{
    NotCandidateDetail, PreHeaderConnectKind, RetryDecision, RetryFailureInput, RetryGiveUpReason,
    RetrySettings, decide_retry,
};

const SETTINGS: RetrySettings = RetrySettings {
    retries: 3,
    backoff_min: Duration::from_secs(1),
    backoff_max: Duration::from_secs(30),
    retry_after_max: Duration::from_secs(60),
};

fn fixed_now() -> SystemTime {
    // 2015-10-21 07:28:00 GMT
    UNIX_EPOCH + Duration::from_secs(1_445_412_480)
}

fn http(status: u16) -> RetryFailureInput {
    RetryFailureInput::HttpStatus {
        status,
        has_retry_after: false,
    }
}

fn decide(
    input: RetryFailureInput,
    header: Option<&str>,
    attempt: u32,
    sample: f64,
) -> RetryDecision {
    decide_retry(input, header, attempt, SETTINGS, fixed_now(), sample)
}

#[test]
fn whitelist_with_retry_after_within_cap_retries() {
    let decision = decide(http(429), Some("30"), 1, 0.5);
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::from_secs(30)
        }
    );
}

#[test]
fn http_date_within_cap_retries() {
    // fixed_now = 2015-10-21 07:28:00 GMT → +45초(상한 60초 이하)
    let decision = decide(http(503), Some("Wed, 21 Oct 2015 07:28:45 GMT"), 1, 0.5);
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::from_secs(45)
        }
    );
}

#[test]
fn negative_retry_after_uses_full_jitter_backoff() {
    // 음수 → Absent → Full Jitter (sample=1.0 → backoff_min)
    let decision = decide(http(429), Some("-1"), 1, 1.0);
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::from_secs(1)
        }
    );
}

#[test]
fn unparseable_retry_after_uses_full_jitter_backoff() {
    // 해석불가 → Absent → Full Jitter (sample=1.0 → backoff_min)
    let decision = decide(http(500), Some("soon"), 1, 1.0);
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::from_secs(1)
        }
    );
}

#[test]
fn payment_required_with_retry_after_gives_up_not_candidate() {
    let decision = decide(
        RetryFailureInput::HttpStatus {
            status: 402,
            has_retry_after: true,
        },
        Some("10"),
        1,
        0.5,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp(RetryGiveUpReason::NotCandidate(
            NotCandidateDetail::HttpStatus { status: 402 }
        ))
    );
}

#[test]
fn retry_after_above_cap_gives_up_with_wait() {
    let decision = decide(http(503), Some("120"), 1, 0.5);
    assert_eq!(
        decision,
        RetryDecision::GiveUp(RetryGiveUpReason::RetryAfterExceedsCap {
            wait: Duration::from_secs(120)
        })
    );
}

#[test]
fn absent_header_uses_full_jitter_backoff() {
    let decision = decide(http(500), None, 1, 1.0);
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::from_secs(1)
        }
    );
}

#[test]
fn pre_header_connect_uses_backoff() {
    let decision = decide(
        RetryFailureInput::PreHeaderConnect(PreHeaderConnectKind::DnsFailure),
        None,
        2,
        1.0,
    );
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::from_secs(2)
        }
    );
}

#[test]
fn attempts_exceeded_gives_up() {
    let decision = decide(http(429), None, 4, 0.5);
    assert_eq!(
        decision,
        RetryDecision::GiveUp(RetryGiveUpReason::AttemptsExceeded { attempt: 4 })
    );
}

#[test]
fn attempts_exceeded_preferred_over_retry_after_cap() {
    let decision = decide(http(429), Some("120"), 4, 0.5);
    assert_eq!(
        decision,
        RetryDecision::GiveUp(RetryGiveUpReason::AttemptsExceeded { attempt: 4 })
    );
}

#[test]
fn header_wait_timeout_gives_up_not_candidate() {
    let decision = decide(RetryFailureInput::HeaderWaitTimeout, None, 1, 0.5);
    assert_eq!(
        decision,
        RetryDecision::GiveUp(RetryGiveUpReason::NotCandidate(
            NotCandidateDetail::HeaderWaitTimeout
        ))
    );
}

#[test]
fn zero_retry_after_retries_immediately() {
    let decision = decide(http(429), Some("0"), 1, 0.5);
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::ZERO
        }
    );
}

#[test]
fn settings_are_duration_and_u32_only() {
    // HttpSettings 타입 없이 Duration/u32 구조체만으로 판단이 끝납니다(AC 1).
    let settings = RetrySettings {
        retries: 1_u32,
        backoff_min: Duration::from_millis(100),
        backoff_max: Duration::from_secs(2),
        retry_after_max: Duration::from_secs(5),
    };
    let decision = decide_retry(http(503), Some("3"), 1, settings, fixed_now(), 0.0);
    assert_eq!(
        decision,
        RetryDecision::Retry {
            wait: Duration::from_secs(3)
        }
    );
}

#[test]
fn give_up_reason_debug_has_no_secrets_host_or_body() {
    let reasons = [
        RetryGiveUpReason::NotCandidate(NotCandidateDetail::HttpStatus { status: 402 }),
        RetryGiveUpReason::NotCandidate(NotCandidateDetail::HeaderWaitTimeout),
        RetryGiveUpReason::AttemptsExceeded { attempt: 4 },
        RetryGiveUpReason::RetryAfterExceedsCap {
            wait: Duration::from_secs(120),
        },
    ];
    let forbidden = [
        "sk-",
        "api_key",
        "Authorization",
        "example.com",
        "base_url",
        "\"error\"",
        "response body",
        "https://",
    ];
    for reason in reasons {
        let rendered = format!("{reason:?}");
        for needle in forbidden {
            assert!(
                !rendered.contains(needle),
                "포기 이유 Debug에 금지 항목이 있습니다: {needle} in {rendered}"
            );
        }
    }
}
