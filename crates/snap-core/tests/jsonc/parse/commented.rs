//! T1 공용 JSONC 해석기 시험. 주석·쉼표 없는 텍스트를 해석하는 책임만 봄.
use snap_core::jsonc::parse_jsonc;

#[test]
fn parses_commented_jsonc() {
    let text = "{\n  // comment\n \"api_keys\": { \"primary\": \"primary-value\" }\n}";

    let value = parse_jsonc(text).expect("test builds commented jsonc");

    let keys = value.get("api_keys").expect("test reads api_keys");
    assert_eq!(keys["primary"].as_str(), Some("primary-value"));
}
