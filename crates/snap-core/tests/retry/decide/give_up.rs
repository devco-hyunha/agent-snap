//! `decide_retry`의 즉시 포기 시험. spec AC 5·6의 대상 아님·횟수 없는 포기를 확인함.

use snap_core::llm::FailureInput;
use snap_core::llm::RetryDecision;
use snap_core::llm::RetryReason;
use snap_core::llm::decide_retry;

use crate::fixtures::FixedSource;
use crate::fixtures::NOW;
use crate::fixtures::SETTINGS;

/// 402·408·425·화이트리스트 밖 5xx·그 밖의 4xx(429 제외)는 바로 포기.
#[test]
fn blacklist_status_is_give_up() {
    let codes = [402_u16, 403, 404, 408, 409, 425, 451, 501, 505, 507, 599];
    for code in codes {
        let mut rng = FixedSource { value: 0 };
        let decision = decide_retry(
            FailureInput::Status(code),
            None,
            1,
            &SETTINGS,
            NOW,
            &mut rng,
        );
        let expected = RetryDecision::GiveUp {
            reason: RetryReason::NotRetryable {
                input: FailureInput::Status(code),
            },
        };
        assert_eq!(decision, expected, "상태 코드 {code}");
    }
}

/// 402는 `Retry-After`가 있어도 재시도하지 않음.
#[test]
fn status_402_with_header_is_give_up() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::Status(402),
        Some("30"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp {
            reason: RetryReason::NotRetryable {
                input: FailureInput::Status(402),
            }
        }
    );
}

/// 헤더 대기 초과는 재시도 대상이 아니라 바로 포기.
#[test]
fn header_wait_timeout_is_give_up() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::HeaderWaitTimeout,
        Some("30"),
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp {
            reason: RetryReason::NotRetryable {
                input: FailureInput::HeaderWaitTimeout,
            }
        }
    );
}

/// 포기 이유의 대상 아님 칸은 입력 종류를 그대로 담음.
#[test]
fn not_retryable_reason_carries_input() {
    let mut rng = FixedSource { value: 0 };
    let decision = decide_retry(
        FailureInput::HeaderWaitTimeout,
        None,
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::GiveUp {
            reason: RetryReason::NotRetryable {
                input: FailureInput::HeaderWaitTimeout,
            }
        }
    );
}
