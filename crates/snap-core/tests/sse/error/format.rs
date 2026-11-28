//! 02-3 T4 형식 오류 시험. 잘못된 JSON 청크와 잘린 마지막 이벤트의 위치·종류만 봄.

use snap_core::llm::ChunkErrorKind;
use snap_core::llm::SseError;
use snap_core::llm::StreamState;

const SECRET_MARK: &str = "sk-secretvalue123";
const HOST_MARK: &str = "https://api.example-host.test";

#[test]
fn invalid_json_chunk_is_format_error() {
    let mut stream = StreamState::new();
    let bytes = b"data: {not json\n\n";

    #[allow(clippy::unwrap_used)]
    let error = stream.push(bytes).unwrap_err();

    let SseError::Format { index, kind } = &error else {
        panic!("invalid JSON must make a format error: {error:?}");
    };
    assert_eq!(*index, 1, "event number is reported");
    assert_eq!(*kind, ChunkErrorKind::Syntax, "kind is reported");
}

#[test]
fn format_error_counts_the_event_number() {
    let mut stream = StreamState::new();
    let good = b"data: {\"choices\":[{\"delta\":{\"content\":\"a\"}}]}\n\n";
    let bad = b"data: {oops\n\n";

    #[allow(clippy::unwrap_used)]
    stream.push(good).unwrap();
    #[allow(clippy::unwrap_used)]
    let error = stream.push(bad).unwrap_err();

    let SseError::Format { index, .. } = &error else {
        panic!("format error expected: {error:?}");
    };
    assert_eq!(*index, 2, "the second event is the broken one");
}

#[test]
fn truncated_last_event_is_format_error() {
    let mut stream = StreamState::new();
    let head = b"data: {\"choices\":[{\"delta\":{\"content\":\"a\"}}}";

    #[allow(clippy::unwrap_used)]
    stream.push(head).unwrap();
    #[allow(clippy::unwrap_used)]
    let error = stream.finish().unwrap_err();

    let SseError::Format { index, kind } = &error else {
        panic!("truncated tail must make a format error: {error:?}");
    };
    assert_eq!(*index, 1, "the tail event number is reported");
    assert_eq!(*kind, ChunkErrorKind::Syntax, "kind is reported");
}

#[test]
fn truncated_multibyte_tail_is_format_error() {
    let mut stream = StreamState::new();
    // `data: 가`의 문자 한가운데(8바이트)에서 잘라 넣기.
    let bytes = "data: 가".as_bytes()[..8].to_vec();

    #[allow(clippy::unwrap_used)]
    stream.push(&bytes).unwrap();
    #[allow(clippy::unwrap_used)]
    let error = stream.finish().unwrap_err();

    let SseError::Format { index, kind } = &error else {
        panic!("truncated multibyte tail must make a format error: {error:?}");
    };
    assert_eq!(*index, 1, "tail event number is reported");
    assert_eq!(
        *kind,
        ChunkErrorKind::IoOrEof,
        "kind is reported without chunk text"
    );
}

#[test]
fn format_error_carries_no_chunk_text_key_or_host() {
    let mut stream = StreamState::new();
    let broken = format!("data: {{\"key\": \"{SECRET_MARK}\", \"url\": \"{HOST_MARK}\"");
    let bytes = broken.into_bytes();

    #[allow(clippy::unwrap_used)]
    stream.push(&bytes).unwrap();
    #[allow(clippy::unwrap_used)]
    let error = stream.finish().unwrap_err();

    let shown = format!("{error} {error:?}");
    assert!(
        !shown.contains(SECRET_MARK) && !shown.contains(HOST_MARK),
        "format error must not carry chunk text, key or host: {shown}"
    );
}
