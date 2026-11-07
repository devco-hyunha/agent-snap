//! 재시도 대상 판별 단위 테스트(AC 1~4, spec AC 5·6).

use crate::llm::retry::classify::{RetryClass, classify_retry};
use crate::llm::retry::{PreHeaderConnectKind, RetryFailureInput};

fn assert_candidate(input: RetryFailureInput) {
    assert_eq!(classify_retry(input), RetryClass::Candidate);
    assert!(classify_retry(input).is_candidate());
}

fn assert_not_candidate(input: RetryFailureInput) {
    assert_eq!(classify_retry(input), RetryClass::NotCandidate);
    assert!(!classify_retry(input).is_candidate());
}

fn status(code: u16, has_retry_after: bool) -> RetryFailureInput {
    RetryFailureInput::HttpStatus {
        status: code,
        has_retry_after,
    }
}

#[test]
fn whitelist_statuses_are_retry_candidates() {
    for code in [429_u16, 500, 502, 503, 504] {
        assert_candidate(status(code, false));
        assert_candidate(status(code, true));
    }
}

#[test]
fn blacklist_statuses_are_not_candidates() {
    for code in [
        400_u16, 401, 402, 403, 404, 408, 418, 425, 451, 501, 505, 511,
    ] {
        assert_not_candidate(status(code, false));
    }
}

#[test]
fn payment_required_with_retry_after_is_still_not_candidate() {
    assert_not_candidate(status(402, true));
}

#[test]
fn pre_header_connect_kinds_are_candidates() {
    let kinds = [
        PreHeaderConnectKind::DnsFailure,
        PreHeaderConnectKind::ConnectionRefused,
        PreHeaderConnectKind::ConnectTimeout,
        PreHeaderConnectKind::ResetBeforeHeaders,
        PreHeaderConnectKind::Tls,
        PreHeaderConnectKind::Proxy,
    ];
    for kind in kinds {
        assert_candidate(RetryFailureInput::PreHeaderConnect(kind));
    }
}

#[test]
fn header_wait_timeout_is_not_candidate() {
    assert_not_candidate(RetryFailureInput::HeaderWaitTimeout);
}
