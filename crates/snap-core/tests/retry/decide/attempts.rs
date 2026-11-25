//! `decide_retry`의 시도 횟수 초과 시험. spec AC 3의 경계와 AC 4의 우선순위를 확인함.

use std::time::Duration;

use snap_core::llm::FailureInput;
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::RetryDecision;
use snap_core::llm::RetryReason;
use snap_core::llm::decide_retry;

use crate::fixtures::FixedSource;
use crate::fixtures::NOW;
use crate::fixtures::SETTINGS;

/// 최대 재시도 3회에서 시도 4는 지수 대기로 대기.
#[test]
fn last_allowed_attempt_still_waits() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::PreHeaderConnect(PreHeaderConnectKind::Proxy),
        None,
        4,
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

/// 시도 5는 횟수 초과로 포기.
#[test]
fn attempt_over_limit_is_give_up() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::PreHeaderConnect(PreHeaderConnectKind::Proxy),
        None,
        5,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp {
            reason: RetryReason::AttemptLimit { attempt: 5 }
        }
    );
}

/// 횟수 초과와 `Retry-After` 상한 초과가 동시에 해당하면 이유는 횟수 초과.
#[test]
fn attempt_limit_wins_over_retry_after_cap() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::Status(429),
        Some("3600"),
        5,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp {
            reason: RetryReason::AttemptLimit { attempt: 5 }
        }
    );
}

/// 헤더가 상한을 넘는 경우에도 횟수 이내면 상한 초과로 포기.
#[test]
fn within_limit_header_cap_is_give_up() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::Status(429),
        Some("3600"),
        4,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp {
            reason: RetryReason::RetryAfterCap {
                wait: Duration::from_secs(3_600)
            }
        }
    );
}
