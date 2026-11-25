//! 02-3 T4 최상위 `error` 청크의 스트림 오류 시험. 종료 여부와 표시의 가림만 봄.

use snap_core::llm::SseError;
use snap_core::llm::StreamState;

use super::stream_error_from;

const SECRET_MARK: &str = "sk-secretvalue123";
const HOST_MARK: &str = "https://api.example-host.test";

#[test]
fn top_level_error_chunk_ends_stream_with_stream_error() {
    let payload = r#"{"error": {"message": "rate limit reached", "type": "server_error"}}"#;
    let error = stream_error_from(payload);

    assert!(
        matches!(error, SseError::Stream { .. }),
        "top-level error makes a stream error"
    );
}

#[test]
fn stream_error_shows_server_summary_text() {
    let payload = r#"{"error": {"message": "rate limit reached"}}"#;
    let error = stream_error_from(payload);

    let shown = error.to_string();
    assert!(
        shown.contains("rate limit"),
        "summary text stays visible: {shown}"
    );
}

#[test]
fn stream_error_hides_key_and_host_from_display() {
    let payload = format!(r#"{{"error": {{"message": "key={SECRET_MARK} at {HOST_MARK}"}}}}"#,);
    let error = stream_error_from(&payload);

    let shown = error.to_string();
    assert!(
        !shown.contains(SECRET_MARK) && !shown.contains(HOST_MARK),
        "display must not carry key or host: {shown}"
    );
}

#[test]
fn stream_error_hides_key_and_host_from_debug() {
    let payload = format!(r#"{{"error": {{"message": "key={SECRET_MARK} at {HOST_MARK}"}}}}"#,);
    let error = stream_error_from(&payload);

    let shown = format!("{error:?}");
    assert!(
        !shown.contains(SECRET_MARK) && !shown.contains(HOST_MARK),
        "debug must not carry key or host: {shown}"
    );
}

#[test]
fn stream_error_stops_later_chunks() {
    let mut stream = StreamState::new();
    let payload = r#"{"error": {"message": "boom"}}"#;
    let first = format!("data: {payload}\n\n").into_bytes();

    #[allow(clippy::unwrap_used)]
    let _ = stream.push(&first).unwrap_err();
    #[allow(clippy::unwrap_used)]
    stream
        .push(b"data: {\"choices\":[{\"delta\":{\"content\":\"more\"}}]}\n\n")
        .unwrap();

    assert_eq!(
        stream.delta().content,
        "",
        "content after an error is not appended"
    );
}
