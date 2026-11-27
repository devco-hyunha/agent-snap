//! T4 시험: `extra` 병합 책임. 원문 그대로 합치기만 봄.
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use snap_core::llm::EndpointKind;
use snap_core::llm::build_body;

use crate::request::request;
use crate::request::request_with_extra;
use crate::request_body::field;

/// 키와 값이 같은 extra 객체.
fn extra(pairs: &[(&str, Value)]) -> Map<String, Value> {
    let mut map = Map::new();
    for (key, value) in pairs {
        map.insert((*key).to_owned(), value.clone());
    }
    map
}

#[test]
fn extra_merges_as_is_including_nested_object() {
    let body = build_body(
        &request_with_extra(extra(&[
            ("vendor", json!("x")),
            ("nested", json!({"a": 1, "b": [true]})),
        ])),
        EndpointKind::Lmstudio,
    );

    assert_eq!(field(&body, "vendor"), Some(&json!("x")));
    assert_eq!(field(&body, "nested"), Some(&json!({"a": 1, "b": [true]})));
}

#[test]
fn extra_merges_into_every_kind() {
    let kinds = [
        EndpointKind::Openai,
        EndpointKind::Openrouter,
        EndpointKind::Ollama,
        EndpointKind::Vllm,
        EndpointKind::Llamacpp,
        EndpointKind::Lmstudio,
    ];
    for kind in kinds {
        let body = build_body(&request_with_extra(extra(&[("keep", json!(7))])), kind);
        assert_eq!(
            field(&body, "keep"),
            Some(&json!(7)),
            "kind {kind} merges extra"
        );
    }
}

#[test]
fn absent_extra_sends_nothing() {
    let body = build_body(&request(), EndpointKind::Openai);

    let keys: Vec<String> = body
        .as_object()
        .expect("body is object")
        .keys()
        .filter(|key| *key != "model" && *key != "messages" && *key != "stream")
        .cloned()
        .collect();
    assert!(keys.is_empty(), "absent extra adds no keys: {keys:?}");
}
