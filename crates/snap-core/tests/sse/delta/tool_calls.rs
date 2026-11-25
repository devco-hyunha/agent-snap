//! 02-3 T3 도구 호출 index별 누적 시험. index별로 id·이름·인자를 합치는 동작만 봄.

use snap_core::llm::DeltaState;

use super::delta_payload;

#[test]
fn tool_call_delta_pieces_merge_by_index() {
    let mut state = DeltaState::new();
    let payloads = [
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 0, "id": "call_1", "function": { "name": "get_weather", "arguments": "" } }]
        })),
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 0, "function": { "arguments": "{\"ci" } }]
        })),
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 0, "function": { "arguments": "ty\": \"서울\"}" } }]
        })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    let finished = state.finish();
    assert_eq!(
        finished.tool_calls.len(),
        1,
        "one index makes one tool call"
    );
    let tool_call = &finished.tool_calls[0];
    assert_eq!(tool_call.index, 0);
    assert_eq!(tool_call.id.as_deref(), Some("call_1"));
    assert_eq!(tool_call.name, "get_weather");
    assert_eq!(tool_call.arguments, "{\"city\": \"서울\"}");
}

#[test]
fn alternating_index_deltas_merge_per_index() {
    let mut state = DeltaState::new();
    let payloads = [
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 0, "id": "call_a", "function": { "name": "one", "arguments": "A" } }]
        })),
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 1, "id": "call_b", "function": { "name": "two", "arguments": "X" } }]
        })),
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 0, "function": { "arguments": "B" } }]
        })),
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 1, "function": { "arguments": "Y" } }]
        })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    let finished = state.finish();
    assert_eq!(
        finished.tool_calls.len(),
        2,
        "two indexes make two tool calls"
    );
    assert_eq!(finished.tool_calls[0].index, 0);
    assert_eq!(finished.tool_calls[0].id.as_deref(), Some("call_a"));
    assert_eq!(finished.tool_calls[0].name, "one");
    assert_eq!(finished.tool_calls[0].arguments, "AB");
    assert_eq!(finished.tool_calls[1].index, 1);
    assert_eq!(finished.tool_calls[1].id.as_deref(), Some("call_b"));
    assert_eq!(finished.tool_calls[1].name, "two");
    assert_eq!(finished.tool_calls[1].arguments, "XY");
}

#[test]
fn tool_call_id_keeps_only_the_first_value() {
    let mut state = DeltaState::new();
    let payloads = [
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 0, "id": "call_first" } ]
        })),
        delta_payload(serde_json::json!({
            "tool_calls": [{ "index": 0, "id": "call_second" } ]
        })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    let finished = state.finish();
    assert_eq!(
        finished.tool_calls[0].id.as_deref(),
        Some("call_first"),
        "later id deltas do not overwrite the first id"
    );
}

#[test]
fn tool_call_delta_without_index_is_skipped() {
    let mut state = DeltaState::new();
    let payload = delta_payload(serde_json::json!({
        "tool_calls": [{ "function": { "name": "no_index" } }]
    }));

    state.apply(&payload).unwrap();

    assert!(
        state.finish().tool_calls.is_empty(),
        "delta without index makes no tool call"
    );
}
