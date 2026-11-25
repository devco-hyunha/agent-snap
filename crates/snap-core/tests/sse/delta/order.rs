//! 02-3 T3 content 순서 보존 시험. 보낸 순서대로 이어 붙이는 동작만 봄.

use snap_core::llm::DeltaState;
use snap_core::llm::LineState;
use snap_core::llm::SseEvent;

use super::delta_payload;

#[test]
fn content_chunks_append_in_sent_order() {
    let mut state = DeltaState::new();

    for piece in ["a", "bc", "d"] {
        let payload = delta_payload(serde_json::json!({ "content": piece }));
        state.apply(&payload).unwrap();
    }

    assert_eq!(state.finish().content, "abcd", "content keeps sent order");
}

#[test]
fn non_text_content_is_skipped() {
    let mut state = DeltaState::new();
    let payload = delta_payload(serde_json::json!({ "content": 7 }));

    state.apply(&payload).unwrap();

    assert_eq!(
        state.finish().content,
        "",
        "non string content adds nothing"
    );
}

#[test]
fn line_state_and_delta_state_join_stream_content() {
    let mut lines = LineState::new();
    let mut delta = DeltaState::new();
    let first = delta_payload(serde_json::json!({ "content": "가" }));
    let second = delta_payload(serde_json::json!({ "content": "나" }));
    let stream = format!("data: {first}\n\ndata: {second}\n\ndata: [DONE]\n");

    let events = lines.push(stream.as_bytes()).unwrap();
    events.into_iter().for_each(|event| {
        if let SseEvent::Data { payload } = event {
            delta.apply(&payload).unwrap();
        }
    });

    let finished = delta.finish();
    assert_eq!(finished.content, "가나", "stream content joins in order");
}
