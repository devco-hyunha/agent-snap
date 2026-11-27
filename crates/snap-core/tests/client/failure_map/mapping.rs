//! 전송 오류 → 02-4 판단 입력 매핑 표 시험. 02-4 spec 매핑 표의 행을 함수 동작으로 확인함.

use snap_core::llm::ChunkErrorKind;
use snap_core::llm::FailureInput;
use snap_core::llm::RetryAfterHeader;
use snap_core::llm::SseError;
use snap_core::llm::TransportError;
use snap_core::llm::map_failure;

use crate::failure_map::materials::connect_kinds;
use crate::failure_map::materials::summary;

/// 연결 실패 6종은 헤더 전 연결 오류로 옮긴다.
#[test]
fn connect_kinds_map_to_pre_header_connect() {
    for kind in connect_kinds() {
        let error = TransportError::connect(kind, 10.0);
        let mapped = map_failure(&error).expect("connect failure is a decision input");

        assert_eq!(
            mapped,
            FailureInput::PreHeaderConnect(kind),
            "kind carries over"
        );
    }
}

/// 헤더 대기 초과는 비재시도 입력으로 옮긴다.
#[test]
fn header_wait_maps_to_header_wait_timeout() {
    let error = TransportError::header_wait(300.0);
    let mapped = map_failure(&error).expect("header wait is a decision input");

    assert_eq!(
        mapped,
        FailureInput::HeaderWaitTimeout,
        "header wait maps to the non-retry input"
    );
}

/// 재시도 후보 상태 코드는 상태 코드 입력으로 옮긴다.
#[test]
fn retry_status_codes_map_to_status() {
    for code in [429_u16, 500, 502, 503, 504] {
        let error = TransportError::status(code, summary(), None);
        let mapped = map_failure(&error).expect("status is a decision input");

        assert_eq!(mapped, FailureInput::Status(code), "code carries over");
    }
}

/// 재시도 대상 아닌 상태 코드는 상태 코드 입력으로 옮긴다.
#[test]
fn non_retry_status_codes_map_to_status() {
    let codes = [402_u16, 403, 404, 408, 409, 425, 451, 501, 505, 507, 599];
    for code in codes {
        let error = TransportError::status(code, summary(), None);
        let mapped = map_failure(&error).expect("status is a decision input");

        assert_eq!(mapped, FailureInput::Status(code), "code carries over");
    }
}

/// 헤더 이후 조각 읽기 실패도 판단 입력이 아니다.
#[test]
fn stream_read_after_header_maps_to_none() {
    let error = TransportError::stream_read(10.0);

    assert!(
        map_failure(&error).is_none(),
        "post-header read failure is not a decision input"
    );
}

/// 스트림 유휴·스트림 오류·취소·끝난 스트림은 판단 입력이 아니므로 None.
#[test]
fn stream_side_errors_map_to_none() {
    let errors = [
        TransportError::idle(120.0),
        TransportError::stream(SseError::format(2, ChunkErrorKind::Syntax)),
        TransportError::cancelled(),
        TransportError::closed(),
    ];

    for error in errors {
        assert!(
            map_failure(&error).is_none(),
            "not a decision input: {error}"
        );
    }
}

/// 매핑은 헤더 값을 쓰지 않는다. 상태 코드가 같으면 옮긴 입력이 같다.
#[test]
fn mapping_ignores_retry_after_header_value() {
    let with_header =
        TransportError::status(429, summary(), Some(RetryAfterHeader::new("12".to_owned())));
    let without_header = TransportError::status(429, summary(), None);

    let with = map_failure(&with_header).expect("status is a decision input");
    let without = map_failure(&without_header).expect("status is a decision input");

    assert_eq!(with, without, "header value never enters the mapping");
}
