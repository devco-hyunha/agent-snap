//! 02-6 전송 오류 타입 담는 값 시험. 변형이 받는 값과 생성자 전달만 봄.
//! 시험 본문과 단정값은 `tests/client/error.rs`에서 그대로 옮김.

use snap_core::llm::ChunkErrorKind;
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::SseError;
use snap_core::llm::TransportError;

use crate::error::materials::header;
use crate::error::materials::summary;

#[test]
fn connect_error_carries_kind_and_seconds() {
    let error = TransportError::connect(PreHeaderConnectKind::Refused, 10.0);

    assert!(
        matches!(&error, TransportError::Connect { kind, connect_seconds } if
            *kind == PreHeaderConnectKind::Refused && *connect_seconds == 10.0),
        "connect error carries kind and seconds"
    );
}

#[test]
fn status_error_carries_code_and_redacted_summary() {
    let error = TransportError::status(429, summary(), None);

    let TransportError::Status {
        status,
        summary,
        retry_after,
    } = &error
    else {
        panic!("test expects status error");
    };

    assert_eq!(*status, 429, "status carries code");
    assert_eq!(summary.to_string(), "서버이 오류입니다", "carries summary");
    assert!(retry_after.is_none(), "header absent stays none");
    assert!(
        format!("{error:?}").contains("RedactedSummary"),
        "debug uses redacted wrapper label"
    );
}

#[test]
fn status_error_carries_redacted_retry_after_header() {
    let error = TransportError::status(429, summary(), Some(header("12")));

    let TransportError::Status { retry_after, .. } = &error else {
        panic!("test expects status error");
    };

    let header = retry_after.as_ref().expect("test carries header value");
    assert_eq!(header.expose(), "12", "header carries server value");
    assert!(
        format!("{header:?}").contains("RetryAfterHeader(가려짐)"),
        "debug hides header value: {header:?}"
    );
}

#[test]
fn timeout_errors_carry_seconds() {
    let header = TransportError::header_wait(300.0);
    let idle = TransportError::idle(120.0);

    assert!(
        matches!(header, TransportError::HeaderWait { seconds } if seconds == 300.0),
        "header wait carries seconds"
    );
    assert!(
        matches!(idle, TransportError::Idle { seconds } if seconds == 120.0),
        "idle carries seconds"
    );
}

#[test]
fn stream_read_error_carries_connect_seconds() {
    let error = TransportError::stream_read(10.0);

    assert!(
        matches!(error, TransportError::StreamRead { connect_seconds } if connect_seconds == 10.0),
        "post-header read failure carries the connect setting seconds"
    );
    assert!(
        !error.to_string().is_empty(),
        "post-header read failure has text"
    );
}

#[test]
fn stream_error_wraps_sse_error() {
    let inner = SseError::format(2, ChunkErrorKind::Syntax);
    let error = TransportError::stream(inner);

    assert!(
        matches!(&error, TransportError::Stream(SseError::Format { index, .. }) if *index == 2),
        "stream error keeps sse error position"
    );
}

#[test]
fn cancelled_error_prints() {
    let error = TransportError::cancelled();

    assert!(!error.to_string().is_empty(), "cancel error has text");
}
