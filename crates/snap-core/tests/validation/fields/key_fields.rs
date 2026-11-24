//! T3 검증 시험: 자격 키 칸 거부 책임. `api_key`·`api_key_env`만 봄.
use crate::material::HOST;
use crate::material::endpoints;
use crate::material::reject_message;

const ENDPOINT: &str = "primary";
const SECRET_VALUE: &str = "sk-SECRETVALUE";

#[test]
fn rejects_endpoint_api_key_field() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "api_key": "sk-SECRETVALUE" }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains(ENDPOINT),
        "message names endpoint: {message}"
    );
    assert!(
        !message.contains(SECRET_VALUE),
        "message hides key value: {message}"
    );
    assert!(!message.contains(HOST), "message hides address: {message}");
    assert!(
        message.contains("credentials.jsonc"),
        "message points to credentials file: {message}"
    );
}

#[test]
fn rejects_endpoint_api_key_env_field() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "api_key_env": "SNAP_KEY" }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains(ENDPOINT),
        "message names endpoint: {message}"
    );
    assert!(
        !message.contains("SNAP_KEY"),
        "message hides env name value: {message}"
    );
}
