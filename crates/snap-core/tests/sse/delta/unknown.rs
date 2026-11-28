//! 02-3 T3 모르는 델타 필드 원문 보존 시험. 모르는 필드를 JSON 원문으로 남기는 동작만 봄.

use snap_core::llm::DeltaState;

use super::delta_payload;

#[test]
fn unknown_delta_field_keeps_raw_json() {
    let mut state = DeltaState::new();
    let payload = delta_payload(serde_json::json!({
        "content": "a",
        "reasoning_details": [{ "id": "r1", "type": "reasoning.text" }],
    }));

    state.apply(&payload).unwrap();

    let finished = state.finish();
    assert_eq!(finished.content, "a", "known fields still merge");
    let kept = finished
        .unknown_fields
        .get("reasoning_details")
        .expect("unknown field is kept");
    assert_eq!(
        kept,
        &serde_json::json!([{ "id": "r1", "type": "reasoning.text" }]),
        "unknown field keeps its raw JSON value"
    );
}

#[test]
fn stream_state_keeps_reasoning_details_raw_json() {
    let mut stream = snap_core::llm::StreamState::new();
    let payload = delta_payload(serde_json::json!({
        "content": "a",
        "reasoning_details": [{ "id": "r1", "type": "reasoning.text" }],
    }));
    let bytes = format!("data: {payload}\n\n").into_bytes();

    #[allow(clippy::unwrap_used)]
    stream.push(&bytes).unwrap();

    let finished = stream.delta();
    let kept = finished
        .unknown_fields
        .get("reasoning_details")
        .expect("unknown field is kept through the stream");
    assert_eq!(
        kept,
        &serde_json::json!([{ "id": "r1", "type": "reasoning.text" }]),
        "stream path keeps the raw JSON of unknown delta fields"
    );
}

#[test]
fn unknown_delta_field_from_latest_chunk_wins() {
    let mut state = DeltaState::new();
    let payloads = [
        delta_payload(serde_json::json!({ "cache_seed": 1 })),
        delta_payload(serde_json::json!({ "cache_seed": 2 })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    let finished = state.finish();
    assert_eq!(
        finished.unknown_fields.get("cache_seed"),
        Some(&serde_json::json!(2)),
        "the latest chunk value is kept"
    );
}
