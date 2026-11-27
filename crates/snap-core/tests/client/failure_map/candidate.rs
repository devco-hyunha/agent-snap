//! 02-4 판단 입력 후보 판별 시험. 02-4 spec 매핑 표의 후보 칸을 `is_candidate` 동작으로 확인함.

use snap_core::llm::RetryCandidate;
use snap_core::llm::TransportError;
use snap_core::llm::is_candidate;
use snap_core::llm::map_failure;

use crate::failure_map::materials::connect_kinds;
use crate::failure_map::materials::summary;

/// 연결 실패 6종은 표대로 모두 후보.
#[test]
fn connect_kinds_are_all_candidates() {
    for kind in connect_kinds() {
        let error = TransportError::connect(kind, 10.0);
        let mapped = map_failure(&error).expect("connect failure is a decision input");

        assert_eq!(
            is_candidate(mapped),
            RetryCandidate::Yes,
            "table row: candidate"
        );
    }
}

/// 헤더 대기 초과는 표대로 후보가 아님.
#[test]
fn header_wait_timeout_is_not_candidate() {
    let error = TransportError::header_wait(300.0);
    let mapped = map_failure(&error).expect("header wait is a decision input");

    assert_eq!(
        is_candidate(mapped),
        RetryCandidate::No,
        "table row: not candidate"
    );
}

/// 재시도 후보 상태 코드는 표대로 후보.
#[test]
fn retry_status_codes_are_candidates() {
    for code in [429_u16, 500, 502, 503, 504] {
        let error = TransportError::status(code, summary(), None);
        let mapped = map_failure(&error).expect("status is a decision input");

        assert_eq!(
            is_candidate(mapped),
            RetryCandidate::Yes,
            "table row: candidate"
        );
    }
}

/// 재시도 대상 아닌 상태 코드는 표대로 후보가 아님.
#[test]
fn non_retry_status_codes_are_not_candidates() {
    let codes = [402_u16, 403, 404, 408, 409, 425, 451, 501, 505, 507, 599];
    for code in codes {
        let error = TransportError::status(code, summary(), None);
        let mapped = map_failure(&error).expect("status is a decision input");

        assert_eq!(
            is_candidate(mapped),
            RetryCandidate::No,
            "table row: not candidate"
        );
    }
}
