//! 재시도 대상 판별 시험. spec AC 5·6의 화이트·블랙리스트를 네트워크 없이 확인함.

use snap_core::llm::FailureInput;
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::RetryCandidate;
use snap_core::llm::is_candidate;

/// 재시도 후보 상태 코드 5종은 모두 후보.
#[test]
fn whitelist_status_is_candidate() {
    for code in [429_u16, 500, 502, 503, 504] {
        assert_eq!(
            is_candidate(FailureInput::Status(code)),
            RetryCandidate::Yes
        );
    }
}

/// 헤더 전 연결 오류 종류는 모두 후보.
#[test]
fn pre_header_connect_is_candidate() {
    let kinds = [
        PreHeaderConnectKind::Dns,
        PreHeaderConnectKind::Refused,
        PreHeaderConnectKind::ConnectTimeout,
        PreHeaderConnectKind::Reset,
        PreHeaderConnectKind::Tls,
        PreHeaderConnectKind::Proxy,
    ];
    for kind in kinds {
        let input = FailureInput::PreHeaderConnect(kind);
        assert_eq!(is_candidate(input), RetryCandidate::Yes);
    }
}

/// 402·408·425·화이트리스트 밖 5xx·그 밖의 4xx(429 제외)는 즉시 포기.
#[test]
fn blacklist_status_is_not_candidate() {
    let codes = [402_u16, 403, 404, 408, 409, 425, 451, 501, 505, 507, 599];
    for code in codes {
        assert_eq!(is_candidate(FailureInput::Status(code)), RetryCandidate::No);
    }
}

/// 402 후보 제외 확인
#[test]
fn status_402_is_not_candidate() {
    // 대상 판별은 헤더 값을 보지 않음. 402는 헤더 유무와 무관하게 후보가 아님.
    assert_eq!(is_candidate(FailureInput::Status(402)), RetryCandidate::No);
}

/// 헤더 대기 초과는 재시도하지 않고 바로 포기.
#[test]
fn header_wait_timeout_is_not_candidate() {
    assert_eq!(
        is_candidate(FailureInput::HeaderWaitTimeout),
        RetryCandidate::No
    );
}
