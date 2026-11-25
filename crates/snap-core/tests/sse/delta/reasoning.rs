//! 02-3 T3 생각 두 이름 시험. `reasoning_content`와 `reasoning` 둘 다 읽고 순서를 유지하는 동작만 봄.

use snap_core::llm::DeltaState;

use super::delta_payload;

#[test]
fn both_reasoning_names_are_read_in_sent_order() {
    let mut state = DeltaState::new();
    let payloads = [
        delta_payload(serde_json::json!({ "reasoning_content": "a" })),
        delta_payload(serde_json::json!({ "reasoning": "b" })),
        delta_payload(serde_json::json!({ "reasoning_content": "c" })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    assert_eq!(
        state.finish().reasoning,
        "abc",
        "reasoning keeps sent order across both names"
    );
}

#[test]
fn reasoning_and_content_do_not_mix() {
    let mut state = DeltaState::new();
    let payloads = [
        delta_payload(serde_json::json!({ "content": "A", "reasoning": "r" })),
        delta_payload(serde_json::json!({ "content": "B", "reasoning_content": "s" })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    let finished = state.finish();
    assert_eq!(finished.content, "AB", "content keeps its own stream");
    assert_eq!(finished.reasoning, "rs", "reasoning keeps its own stream");
}

#[test]
fn same_chunk_reasoning_names_keep_listed_order() {
    let mut state = DeltaState::new();
    let payload = delta_payload(serde_json::json!({
        "reasoning_content": "x",
        "reasoning": "y",
    }));

    state.apply(&payload).unwrap();

    assert_eq!(
        state.finish().reasoning,
        "xy",
        "reasoning_content comes before reasoning within one chunk"
    );
}
