//! T3 검증 시험: 모르는 칸 무시 책임. 모르는 칸을 두고 읽는 것만 봄.
use crate::material::endpoints;
use crate::material::parse;

#[test]
fn ignores_unknown_fields() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "unknown_box": { "a": 1 } }]"#,
    );

    let config = parse(&text).expect("test ignores unknown fields");

    assert_eq!(config.endpoints.len(), 1);
}
