//! T3 검증 시험: `reasoning` 값 책임. 모르는 값 거부와 4종 허용만 봄.
use crate::material::endpoints;
use crate::material::parse;
use crate::material::reject_message;

const MODEL: &str = "small";

#[test]
fn rejects_unknown_reasoning() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "small", "id": "id-a", "reasoning": "veryhigh" }] }]"#,
    );

    let message = reject_message(&text);

    assert!(message.contains(MODEL), "message names model: {message}");
    assert!(
        !message.contains("veryhigh"),
        "message hides input value: {message}"
    );
}

#[test]
fn accepts_all_four_reasoning_values() {
    let values = ["off", "low", "medium", "high"];
    for (index, value) in values.iter().enumerate() {
        let text = endpoints(
            &r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
                 "models": [{ "name": "MODELNAME", "id": "id-a", "reasoning": "REASONING" }] }]"#
                .replace("MODELNAME", &format!("m{index}"))
                .replace("REASONING", value),
        );

        let config = parse(&text).unwrap_or_else(|e| panic!("test accepts reasoning {value}: {e}"));

        let reasoning = config.endpoints[0].models[0]
            .reasoning
            .as_deref()
            .unwrap_or_default();
        assert_eq!(reasoning, *value);
    }
}
