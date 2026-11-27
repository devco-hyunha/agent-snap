//! T4 시험: `kind`별 `reasoning` 변환 책임. 변환 표와 값 매핑만 봄.
use serde_json::json;
use snap_core::llm::EndpointKind;
use snap_core::llm::ReasoningEffort;
use snap_core::llm::build_body;

use crate::request::request_with_reasoning;
use crate::request_body::field;

#[test]
fn shared_kinds_use_reasoning_effort_field() {
    let kinds = [
        EndpointKind::Openai,
        EndpointKind::Ollama,
        EndpointKind::Vllm,
        EndpointKind::Llamacpp,
    ];
    for kind in kinds {
        let body = build_body(&request_with_reasoning(Some(ReasoningEffort::High)), kind);
        assert_eq!(
            field(&body, "reasoning_effort"),
            Some(&json!("high")),
            "kind {kind} uses flat reasoning_effort"
        );
        assert!(
            field(&body, "reasoning").is_none(),
            "kind {kind} sends no nested reasoning"
        );
    }
}

#[test]
fn openrouter_uses_nested_reasoning_object() {
    let body = build_body(
        &request_with_reasoning(Some(ReasoningEffort::Medium)),
        EndpointKind::Openrouter,
    );

    assert_eq!(
        field(&body, "reasoning"),
        Some(&json!({"effort": "medium"})),
        "openrouter nests effort"
    );
    assert!(
        field(&body, "reasoning_effort").is_none(),
        "openrouter sends no flat field"
    );
}

#[test]
fn lmstudio_sends_no_common_reasoning_field() {
    let body = build_body(
        &request_with_reasoning(Some(ReasoningEffort::High)),
        EndpointKind::Lmstudio,
    );

    assert!(
        field(&body, "reasoning_effort").is_none(),
        "lmstudio sends no flat field"
    );
    assert!(
        field(&body, "reasoning").is_none(),
        "lmstudio sends no nested object"
    );
}

#[test]
fn off_maps_to_none_value() {
    let cases: [(EndpointKind, &str); 5] = [
        (EndpointKind::Openai, "none"),
        (EndpointKind::Ollama, "none"),
        (EndpointKind::Vllm, "none"),
        (EndpointKind::Llamacpp, "none"),
        (EndpointKind::Openrouter, "none"),
    ];
    for (kind, expected) in cases {
        let body = build_body(&request_with_reasoning(Some(ReasoningEffort::Off)), kind);
        let value = match kind {
            EndpointKind::Openrouter => field(&body, "reasoning")
                .expect("openrouter sends object")
                .get("effort")
                .expect("effort key")
                .clone(),
            _ => field(&body, "reasoning_effort")
                .expect("flat field")
                .clone(),
        };
        assert_eq!(value, json!(expected), "off maps to {expected}");
    }
}

#[test]
fn low_and_medium_and_high_keep_names() {
    let efforts = [
        (ReasoningEffort::Low, "low"),
        (ReasoningEffort::Medium, "medium"),
        (ReasoningEffort::High, "high"),
    ];
    for (effort, expected) in efforts {
        let body = build_body(&request_with_reasoning(Some(effort)), EndpointKind::Openai);
        assert_eq!(
            field(&body, "reasoning_effort"),
            Some(&json!(expected)),
            "effort keeps its name"
        );
    }
}

#[test]
fn absent_reasoning_sends_no_field() {
    let kinds = [
        EndpointKind::Openai,
        EndpointKind::Openrouter,
        EndpointKind::Ollama,
        EndpointKind::Vllm,
        EndpointKind::Llamacpp,
        EndpointKind::Lmstudio,
    ];
    for kind in kinds {
        let body = build_body(&request_with_reasoning(None), kind);
        assert!(
            field(&body, "reasoning_effort").is_none() && field(&body, "reasoning").is_none(),
            "kind {kind} sends no reasoning field when absent"
        );
    }
}
