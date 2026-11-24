//! T3 검증 시험: 이름 중복 거부 책임. 엔드포인트·모델 이름 중복만 봄.
use crate::material::HOST;
use crate::material::endpoints;
use crate::material::parse;
use crate::material::reject_message;

const ENDPOINT: &str = "primary";
const MODEL: &str = "small";

#[test]
fn rejects_duplicate_endpoint_name_after_trim() {
    let text = endpoints(
        r#"[{ "name": " primary ", "base_url": "https://secret-host.invalid", "kind": "ollama" },
            { "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama" }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains(ENDPOINT),
        "message names endpoint: {message}"
    );
    assert!(!message.contains(HOST), "message hides address: {message}");
}

#[test]
fn rejects_duplicate_model_name_in_same_endpoint() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": " small ", "id": "id-a" }, { "name": "small", "id": "id-b" }] }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains(MODEL) && message.contains(ENDPOINT),
        "message names endpoint and model: {message}"
    );
    assert!(
        !message.contains("id-a"),
        "message hides id value: {message}"
    );
}

#[test]
fn allows_same_model_name_in_different_endpoints() {
    let text = endpoints(
        r#"[{ "name": "one", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "small", "id": "id-a" }] },
            { "name": "two", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "small", "id": "id-b" }] }]"#,
    );

    let config = parse(&text).expect("test allows same alias across endpoints");

    assert_eq!(config.endpoints.len(), 2);
}
