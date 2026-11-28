//! T3 검증 시험: 누락 칸 거부 책임. 이름 칸이 아예 없는 경우만 봄.
use crate::material::HOST;
use crate::material::endpoints;
use crate::material::reject_message;

const ENDPOINT: &str = "primary";

#[test]
fn rejects_missing_endpoint_name_field() {
    let text = endpoints(r#"[{ "base_url": "https://secret-host.invalid", "kind": "ollama" }]"#);

    let message = reject_message(&text);

    assert!(
        message.contains("엔드포인트") || message.contains("endpoint"),
        "message names endpoint slot: {message}"
    );
    assert!(!message.contains(HOST), "message hides address: {message}");
}

#[test]
fn rejects_missing_model_name_field() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "id": "id-a" }] }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains(ENDPOINT),
        "message names endpoint: {message}"
    );
    assert!(
        !message.contains("id-a"),
        "message hides id value: {message}"
    );
}
