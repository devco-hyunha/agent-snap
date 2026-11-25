//! 02-3 T5 사용량 읽기 시험. 최상위 `usage`를 읽고 값으로 쌓는 동작만 봄.
//!
//! `finish_reason` 되풀이의 내용 중복 방지는 `delta/finish.rs`가 본다.

use snap_core::llm::DeltaState;
use snap_core::llm::Usage;

use super::delta_payload;
use super::payload_with;

/// 최상위 `usage`를 골격에 얹은 payload JSON 문자열.
fn usage_payload(usage: serde_json::Value) -> String {
    payload_with(serde_json::json!({ "usage": usage }))
}

#[test]
fn usage_from_chunk_is_read_as_numbers() {
    let mut state = DeltaState::new();
    let payload = usage_payload(serde_json::json!({
        "prompt_tokens": 11, "completion_tokens": 7, "total_tokens": 18
    }));

    state.apply(&payload).unwrap();

    let finished = state.finish();
    let usage = finished.usage.expect("usage is read from the chunk");
    assert_eq!(
        usage,
        Usage {
            prompt_tokens: Some(11),
            completion_tokens: Some(7),
            total_tokens: Some(18),
        },
        "the three numbers are read as sent"
    );
}

#[test]
fn usage_missing_is_none() {
    let mut state = DeltaState::new();

    state
        .apply(&delta_payload(serde_json::json!({ "content": "a" })))
        .unwrap();

    assert_eq!(state.finish().usage, None, "no usage chunk means no usage");
}

#[test]
fn usage_number_that_is_not_a_number_is_none() {
    let mut state = DeltaState::new();
    let payload = usage_payload(serde_json::json!({
        "prompt_tokens": "many", "completion_tokens": 7, "total_tokens": 18
    }));

    state.apply(&payload).unwrap();

    let usage = state.finish().usage.expect("usage object is read");
    assert_eq!(
        usage.prompt_tokens, None,
        "non number prompt_tokens means no value"
    );
    assert_eq!(usage.completion_tokens, Some(7), "valid numbers stay");
}

#[test]
fn usage_from_latest_chunk_wins() {
    let mut state = DeltaState::new();
    let payloads = [
        usage_payload(serde_json::json!({
            "prompt_tokens": 1, "completion_tokens": 1, "total_tokens": 2
        })),
        usage_payload(serde_json::json!({
            "prompt_tokens": 3, "completion_tokens": 4, "total_tokens": 7
        })),
    ];

    for payload in payloads {
        state.apply(&payload).unwrap();
    }

    let usage = state.finish().usage.expect("last usage is kept");
    assert_eq!(
        usage,
        Usage {
            prompt_tokens: Some(3),
            completion_tokens: Some(4),
            total_tokens: Some(7),
        },
        "the last usage chunk replaces the earlier one"
    );
}

#[test]
fn usage_is_not_mixed_into_unknown_fields() {
    let mut state = DeltaState::new();
    let payload = usage_payload(serde_json::json!({ "prompt_tokens": 5 }));

    state.apply(&payload).unwrap();

    let finished = state.finish();
    assert!(
        finished.unknown_fields.is_empty(),
        "top level usage does not enter delta unknown fields"
    );
}
