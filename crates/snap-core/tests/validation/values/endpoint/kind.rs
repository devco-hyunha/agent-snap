//! T3 검증 시험: `kind` 값 책임. 모르는 값 거부와 6종 허용만 봄.
use crate::material::endpoints;
use crate::material::parse;
use crate::material::reject_message;

const ENDPOINT: &str = "primary";

#[test]
fn rejects_unknown_kind() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "unknownserver" }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains(ENDPOINT),
        "message names endpoint: {message}"
    );
    assert!(
        !message.contains("unknownserver"),
        "message hides input value: {message}"
    );
}

#[test]
fn accepts_all_six_kinds() {
    let kinds = [
        "openai",
        "openrouter",
        "ollama",
        "vllm",
        "llamacpp",
        "lmstudio",
    ];
    for (index, kind) in kinds.iter().enumerate() {
        let text = format!(
            r#"{{ "endpoints": [{{ "name": "ep{index}", "base_url": "https://secret-host.invalid", "kind": "{kind}" }}] }}"#
        );

        let config = parse(&text).unwrap_or_else(|e| panic!("test accepts kind {kind}: {e}"));

        assert_eq!(config.endpoints[0].kind, *kind);
    }
}
