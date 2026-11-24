//! T3 검증 시험: 빈 값 거부 책임. trim 뒤 빈 이름·빈 id만 봄.
use crate::material::HOST;
use crate::material::endpoints;
use crate::material::reject_message;

const ENDPOINT: &str = "primary";
const MODEL: &str = "small";

#[test]
fn rejects_empty_endpoint_name_after_trim() {
    let text = endpoints(
        r#"[{ "name": "   ", "base_url": "https://secret-host.invalid", "kind": "ollama" }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains("엔드포인트") || message.contains("endpoint"),
        "message names endpoint slot: {message}"
    );
    assert!(!message.contains(HOST), "message hides address: {message}");
}

#[test]
fn rejects_empty_model_name_after_trim() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": " ", "id": "id-a" }] }]"#,
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

#[test]
fn rejects_empty_model_id_after_trim() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "small", "id": "  " }] }]"#,
    );

    let message = reject_message(&text);

    assert!(message.contains(MODEL), "message names model: {message}");
    assert!(!message.contains(HOST), "message hides address: {message}");
}
