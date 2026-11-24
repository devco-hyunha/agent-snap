//! T3 검증 시험: 이름 trim 저장 규칙. 저장 시 trim 결과를 보는 책임을 둠.
use crate::material::endpoints;
use crate::material::parse;

#[test]
fn trims_names_for_storage() {
    let text = endpoints(
        r#"[{ "name": " primary ", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": " small ", "id": " id-a " }] }]"#,
    );

    let config = parse(&text).expect("test reads trimmed names");

    assert_eq!(config.endpoints[0].name, "primary");
    assert_eq!(config.endpoints[0].models[0].name, "small");
}
