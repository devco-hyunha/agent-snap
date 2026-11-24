//! T3 검증 시험: 구분자 거부 책임. 이름 안 `/`만 봄.
use crate::material::HOST;
use crate::material::endpoints;
use crate::material::reject_message;

#[test]
fn rejects_slash_in_endpoint_name() {
    let text = endpoints(
        r#"[{ "name": "a/b", "base_url": "https://secret-host.invalid", "kind": "ollama" }]"#,
    );

    let message = reject_message(&text);

    assert!(!message.contains(HOST), "message hides address: {message}");
    assert!(
        message.contains("a/b"),
        "message names the endpoint: {message}"
    );
}

#[test]
fn rejects_slash_in_model_name() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "a/b", "id": "id-a" }] }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains("a/b"),
        "message names the model: {message}"
    );
    assert!(
        !message.contains("id-a"),
        "message hides id value: {message}"
    );
}
