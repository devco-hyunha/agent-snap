//! 02-3 T5 `finish_reason` 되풀이 시험. 되풀이 청크의 내용 중복 방지만 봄.

use snap_core::llm::DeltaState;
use snap_core::llm::StreamState;

use super::delta_payload;
use super::payload_with;

#[test]
fn finish_reason_repeat_with_empty_delta_does_not_duplicate_content() {
    let mut state = DeltaState::new();
    let payloads = [
        delta_payload(serde_json::json!({ "content": "가나다" })),
        payload_with(serde_json::json!({
            "finish_reason": "stop",
            "usage": { "prompt_tokens": 4, "completion_tokens": 2, "total_tokens": 6 }
        })),
        payload_with(serde_json::json!({
            "finish_reason": "stop",
            "usage": { "prompt_tokens": 4, "completion_tokens": 2, "total_tokens": 6 }
        })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    let finished = state.finish();
    assert_eq!(
        finished.content, "가나다",
        "usage and finish_reason repeats add no content"
    );
    assert_eq!(finished.reasoning, "", "no reasoning is added by repeats");
    assert!(
        finished.tool_calls.is_empty(),
        "no tool call is added by repeats"
    );
    assert_eq!(
        finished.usage.expect("usage is read").total_tokens,
        Some(6),
        "usage is read at the end"
    );
}

#[test]
fn finish_reason_repeat_through_stream_keeps_content_once() {
    let mut stream = StreamState::new();
    let content_chunk = delta_payload(serde_json::json!({ "content": "ab" }));
    let accounting_chunk = payload_with(serde_json::json!({
        "finish_reason": "stop",
        "usage": { "prompt_tokens": 2, "completion_tokens": 2, "total_tokens": 4 }
    }));
    let stream_text = format!(
        "data: {content_chunk}\n\ndata: {accounting_chunk}\n\ndata: {accounting_chunk}\n\ndata: [DONE]\n"
    );

    stream.push(stream_text.as_bytes()).unwrap();
    stream.finish().unwrap();

    let finished = stream.delta();
    assert_eq!(
        finished.content, "ab",
        "stream content appears exactly once after accounting chunks"
    );
    assert_eq!(
        finished.usage.expect("usage is read").completion_tokens,
        Some(2),
        "accounting usage is read from the stream"
    );
}
