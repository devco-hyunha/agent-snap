//! T3 검증 시험: 옛 칸 거부 책임. 맨 위 `models`·모델 `endpoint`만 봄.
use crate::material::HOST;
use crate::material::endpoints;
use crate::material::reject_message;

#[test]
fn rejects_legacy_top_level_models() {
    let text = r#"{ "models": [{ "name": "small", "id": "id-a" }] }"#;

    let message = reject_message(text);

    assert!(
        message.contains("endpoints[].models"),
        "message points to nested place: {message}"
    );
    assert!(
        !message.contains("id-a"),
        "message hides model value: {message}"
    );
}

#[test]
fn rejects_model_endpoint_field() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "small", "id": "id-a", "endpoint": "other" }] }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains("endpoint"),
        "message names the field: {message}"
    );
    assert!(
        !message.contains("other"),
        "message hides field value: {message}"
    );
    assert!(!message.contains(HOST), "message hides address: {message}");
}
