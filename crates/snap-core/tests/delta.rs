//! 델타 복원 단위 테스트(AC 2, AC 3, AC 5).

mod common;
use common::support::{
    content_events, data_line, decode_all, mixed_stream_bytes, reasoning_events,
};
use snap_core::llm::{StreamEvent, ToolCall, Usage};

#[test]
fn comments_content_reasoning_and_tool_calls_restore_in_order() {
    let bytes = mixed_stream_bytes();
    let (events, assembled) = decode_all(&bytes).unwrap();
    assert_eq!(content_events(&events), "안녕");
    assert_eq!(reasoning_events(&events), "생각");
    assert_eq!(assembled.content, "안녕");
    assert_eq!(assembled.reasoning, "생각");
    assert_eq!(
        assembled.tool_calls.get(&0),
        Some(&ToolCall {
            id: "call_a".to_string(),
            name: "alpha".to_string(),
            arguments: "{\"x\":1}".to_string(),
        })
    );
    assert_eq!(
        assembled.tool_calls.get(&1),
        Some(&ToolCall {
            id: "call_b".to_string(),
            name: "beta".to_string(),
            arguments: "{\"y\":1}".to_string(),
        })
    );
    assert!(events.iter().any(|e| matches!(e, StreamEvent::Done)));
}

#[test]
fn interleaved_parallel_tool_calls_accumulate_by_index() {
    let mut body = String::new();
    body.push_str(&data_line(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"id":"a","function":{"name":"f0","arguments":"{"}}]}}]}"#,
    ));
    body.push_str(&data_line(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":1,"id":"b","function":{"name":"f1","arguments":"["}}]}}]}"#,
    ));
    body.push_str(&data_line(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"function":{"arguments":"\"x\":1"}},{"index":1,"function":{"arguments":"2]"}}]}}]}"#,
    ));
    body.push_str(&data_line(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"function":{"arguments":"}"}}]}}]}"#,
    ));
    let (_, assembled) = decode_all(body.as_bytes()).unwrap();
    assert_eq!(assembled.tool_calls[&0].id, "a");
    assert_eq!(assembled.tool_calls[&0].name, "f0");
    assert_eq!(assembled.tool_calls[&0].arguments, "{\"x\":1}");
    assert_eq!(assembled.tool_calls[&1].id, "b");
    assert_eq!(assembled.tool_calls[&1].name, "f1");
    assert_eq!(assembled.tool_calls[&1].arguments, "[2]");
}

#[test]
fn usage_present_returns_values() {
    let body = data_line(
        r#"{"choices":[{"delta":{}}],"usage":{"prompt_tokens":1,"completion_tokens":2,"total_tokens":3}}"#,
    );
    let (events, assembled) = decode_all(body.as_bytes()).unwrap();
    assert_eq!(
        assembled.usage,
        Some(Usage {
            prompt_tokens: Some(1),
            completion_tokens: Some(2),
            total_tokens: Some(3),
        })
    );
    assert!(events.iter().any(|e| matches!(e, StreamEvent::Usage(_))));
}

#[test]
fn usage_absent_is_none() {
    let body = data_line(r#"{"choices":[{"delta":{"content":"x"}}]}"#);
    let (_, assembled) = decode_all(body.as_bytes()).unwrap();
    assert!(assembled.usage.is_none());
}

#[test]
fn one_delta_keeps_reasoning_field_insertion_order() {
    // 사전순이면 reasoning이 먼저지만, 보낸 순서(reasoning_content → reasoning)를 유지한다.
    let body =
        data_line(r#"{"choices":[{"delta":{"reasoning_content":"먼저","reasoning":"나중"}}]}"#);
    let (events, assembled) = decode_all(body.as_bytes()).unwrap();
    assert_eq!(reasoning_events(&events), "먼저나중");
    assert_eq!(assembled.reasoning, "먼저나중");
}

#[test]
fn one_delta_keeps_unknown_field_insertion_order() {
    // 사전순이면 a_extra가 먼저지만, 보낸 순서(z_extra → a_extra)를 유지한다.
    let body = data_line(r#"{"choices":[{"delta":{"z_extra":1,"a_extra":2,"content":"x"}}]}"#);
    let (_, assembled) = decode_all(body.as_bytes()).unwrap();
    assert_eq!(assembled.content, "x");
    assert_eq!(
        assembled.unknown_delta_fields,
        vec![
            ("z_extra".to_string(), "1".to_string()),
            ("a_extra".to_string(), "2".to_string()),
        ]
    );
}
