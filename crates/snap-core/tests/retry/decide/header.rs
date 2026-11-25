//! `decide_retry`의 `Retry-After` 헤더 대기 시험. spec AC 1·2를 초·HTTP 날짜 형식으로 확인함.

use std::time::Duration;

use snap_core::llm::FailureInput;
use snap_core::llm::RetryDecision;
use snap_core::llm::RetryReason;
use snap_core::llm::decide_retry;

use crate::fixtures::FixedSource;
use crate::fixtures::NOW;
use crate::fixtures::SETTINGS;

/// 재시도 대상 429에 상한 이하의 초 형식 헤더가 있으면 그 시간만큼 대기.
#[test]
fn seconds_header_becomes_wait() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::Status(429),
        Some("30"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: Duration::from_secs(30)
        }
    );
}

/// 재시도 대상 500에도 헤더를 따름. HTTP 날짜 형식은 현재 시각 주입으로 계산.
#[test]
fn date_header_becomes_wait() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::Status(500),
        Some("Thu, 01 Jan 1970 00:00:45 GMT"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: Duration::from_secs(45)
        }
    );
}

/// 상한을 넘는 헤더는 대기 초를 밝히고 포기.
#[test]
fn header_over_cap_is_give_up_with_wait() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::Status(429),
        Some("61"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp {
            reason: RetryReason::RetryAfterCap {
                wait: Duration::from_secs(61)
            }
        }
    );
}

/// 헤더 0초는 대기 0초로 바로 재시도.
#[test]
fn zero_header_is_wait_zero() {
    let mut rng = FixedSource { value: 5 };
    let decision = decide_retry(
        FailureInput::Status(503),
        Some("0"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: Duration::ZERO
        }
    );
}

/// 과거 HTTP 날짜는 대기 0초.
#[test]
fn past_date_is_wait_zero() {
    let mut rng = FixedSource { value: 7 };
    let decision = decide_retry(
        FailureInput::Status(503),
        Some("Wed, 31 Dec 1969 23:59:59 GMT"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: Duration::ZERO
        }
    );
}

/// 상한 경계와 같은 헤더는 대기.
#[test]
fn header_at_cap_is_wait() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::Status(429),
        Some("60"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: Duration::from_secs(60)
        }
    );
}
