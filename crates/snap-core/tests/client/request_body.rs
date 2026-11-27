//! T4 시험: 요청 본문 조립 책임. 필드 채움과 금지 키만 봄.
use serde_json::Value;
use serde_json::json;
use snap_core::llm::EndpointKind;
use snap_core::llm::build_body;

use crate::request::MODEL_ID;
use crate::request::request_with_user_message;
use crate::request::sampling_with_values;

/// 본문에서 필드 하나만 꺼냄. 같은 시험 묶음의 변환·병합 시험이 함께 쓴다.
#[allow(clippy::expect_used)]
pub(crate) fn field<'a>(body: &'a Value, key: &str) -> Option<&'a Value> {
    body.as_object().expect("body is object").get(key)
}

#[test]
fn body_carries_model_messages_stream() {
    let body = build_body(&request_with_user_message(), EndpointKind::Openai);

    assert_eq!(field(&body, "model"), Some(&json!(MODEL_ID)));
    assert_eq!(
        field(&body, "messages"),
        Some(&json!([{"role": "user", "content": "hello"}])),
        "messages keep role and content"
    );
    assert_eq!(field(&body, "stream"), Some(&json!(true)));
}

#[test]
fn body_omits_tools_when_empty() {
    let body = build_body(&request_with_user_message(), EndpointKind::Openai);

    assert!(
        field(&body, "tools").is_none(),
        "empty tools sends no field"
    );
}

#[test]
fn body_carries_tools_when_present() {
    let mut with_tools = request_with_user_message();
    with_tools.tools = vec![json!({"type": "function"})];

    let body = build_body(&with_tools, EndpointKind::Openai);

    assert_eq!(
        field(&body, "tools"),
        Some(&json!([{"type": "function"}])),
        "tools pass through as received"
    );
}

#[test]
fn body_carries_only_present_sampling() {
    let mut with_sampling = request_with_user_message();
    with_sampling.sampling = sampling_with_values();

    let body = build_body(&with_sampling, EndpointKind::Openai);

    assert_eq!(field(&body, "temperature"), Some(&json!(0.5)));
    assert!(
        field(&body, "top_p").is_none(),
        "absent sampling sends no field"
    );
    assert_eq!(field(&body, "max_tokens"), Some(&json!(64)));
}

#[test]
fn body_never_carries_forbidden_fields() {
    let kinds = [
        EndpointKind::Openai,
        EndpointKind::Openrouter,
        EndpointKind::Ollama,
        EndpointKind::Vllm,
        EndpointKind::Llamacpp,
        EndpointKind::Lmstudio,
    ];
    let forbidden = [
        "tool_choice",
        "n",
        "stream_options",
        "context_window",
        "logit_bias",
    ];
    for kind in kinds {
        let body = build_body(&request_with_user_message(), kind);
        let keys: Vec<&str> = forbidden
            .into_iter()
            .filter(|key| field(&body, key).is_some())
            .collect();
        assert!(keys.is_empty(), "kind {kind} sends no forbidden field");
    }
}
