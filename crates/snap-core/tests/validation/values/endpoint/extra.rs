//! T3 검증 시험: `extra` 값 책임. 예약 키 거부와 그 밖의 키 허용만 봄.
use crate::material::endpoints;
use crate::material::parse;
use crate::material::reject_message;

const MODEL: &str = "small";

#[test]
fn rejects_reserved_extra_keys() {
    let keys = ["model", "messages", "stream", "tools"];
    for key in keys {
        let text = endpoints(
            &r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
                 "models": [{ "name": "small", "id": "id-a", "extra": { "EXTRAKEY": "extravalue" } }] }]"#
                .replace("EXTRAKEY", key),
        );

        let message = reject_message(&text);

        assert!(
            message.contains(MODEL) && message.contains(key),
            "message names model and key: {message}"
        );
        assert!(
            !message.contains("extravalue"),
            "message hides extra value: {message}"
        );
    }
}

#[test]
fn accepts_extra_with_other_keys() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "small", "id": "id-a", "extra": { "vendor": { "deep": 1 } } }] }]"#,
    );

    let config = parse(&text).expect("test accepts extra with other keys");

    assert_eq!(config.endpoints[0].models.len(), 1);
}
