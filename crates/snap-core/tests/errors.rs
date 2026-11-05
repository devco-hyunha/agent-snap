//! 오류 종류·모르는 필드 단위 테스트(AC 4). 가림 정책은 `redact` 테스트를 봅니다.

mod common;
use common::support::{data_line, decode_all};
use snap_core::llm::{FormatErrorKind, StreamParseError};

#[test]
fn unknown_delta_fields_keep_raw_json() {
    let body =
        data_line(r#"{"choices":[{"delta":{"content":"a","reasoning_details":[{"id":"r1"}]}}]}"#);
    let (_, assembled) = decode_all(body.as_bytes()).unwrap();
    assert_eq!(assembled.content, "a");
    assert_eq!(assembled.unknown_delta_fields.len(), 1);
    assert_eq!(assembled.unknown_delta_fields[0].0, "reasoning_details");
    assert!(assembled.unknown_delta_fields[0].1.contains("r1"));
}

#[test]
fn top_level_error_becomes_stream_error_without_secrets() {
    let secret = "sk-leak-key-value-1234567890abcd";
    let host = "https://evil.example/v1/chat";
    let body = data_line(&format!(
        r#"{{"error":{{"message":"fail {secret} at {host}"}}}}"#
    ));
    let error = decode_all(body.as_bytes()).unwrap_err();
    assert!(matches!(error, StreamParseError::Stream { .. }));
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains(secret));
    assert!(!debug.contains(secret));
    assert!(!display.contains("evil.example"));
    assert!(!debug.contains("evil.example"));
    assert!(!display.contains(host));
    assert!(!debug.contains(host));
    assert!(!display.contains(body.trim()));
}

#[test]
fn tool_call_index_over_u32_is_format_error() {
    let body = data_line(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":4294967296,"id":"x","function":{"name":"f"}}]}}]}"#,
    );
    let error = decode_all(body.as_bytes()).unwrap_err();
    assert!(matches!(
        error,
        StreamParseError::Format {
            event_index: 1,
            kind: FormatErrorKind::InvalidJson
        }
    ));
}

#[test]
fn invalid_json_is_format_error_without_chunk_body() {
    let chunk = "data: {not-json\n\n";
    let error = decode_all(chunk.as_bytes()).unwrap_err();
    assert!(matches!(
        error,
        StreamParseError::Format {
            event_index: 1,
            kind: FormatErrorKind::InvalidJson
        }
    ));
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("{not-json"));
    assert!(!debug.contains("{not-json"));
    assert!(display.contains("이벤트 1번"));
}

#[test]
fn truncated_last_event_is_format_error() {
    let chunk = "data: {\"choices\":[{\"delta\":{\"content\":\"hi\"";
    let error = decode_all(chunk.as_bytes()).unwrap_err();
    assert!(matches!(
        error,
        StreamParseError::Format {
            event_index: 1,
            kind: FormatErrorKind::TruncatedEvent
        }
    ));
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("choices"));
    assert!(!debug.contains("choices"));
    assert!(!display.contains("\"hi\""));
    assert!(!debug.contains("\"hi\""));
}
