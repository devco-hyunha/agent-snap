//! T2 엔드포인트 수정 시험: 대상 선택 책임. 같은 이름 항목만 바뀌고 나머지는 그대로인지 봄(spec AC 3).
use snap_core::config::save::update_endpoint;

use crate::materials::write_config;

/// 엔드포인트 두 개가 있는 설정 텍스트.
const TWO_ENDPOINTS: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    },\n    {\n      \"name\": \"studio\",\n      \"base_url\": \"http://127.0.0.1:1234/v1\",\n      \"kind\": \"lmstudio\"\n    }\n  ]\n}";

#[test]
fn update_changes_only_named_endpoint() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_ENDPOINTS);

    update_endpoint(&config_dir, "local", "http://127.0.0.1:11435/v1", "ollama")
        .expect("test updates endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    let value = snap_core::jsonc::parse_jsonc(&saved).expect("test parses saved");
    let endpoints = value["endpoints"].as_array().expect("test reads array");
    assert_eq!(endpoints.len(), 2);
    assert_eq!(endpoints[0]["base_url"], "http://127.0.0.1:11435/v1");
    assert_eq!(endpoints[1]["base_url"], "http://127.0.0.1:1234/v1");
    assert_eq!(endpoints[1]["kind"], "lmstudio");
}

#[test]
fn update_keeps_name_of_target() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_ENDPOINTS);

    update_endpoint(&config_dir, "studio", "http://127.0.0.1:1235/v1", "ollama")
        .expect("test updates endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    let value = snap_core::jsonc::parse_jsonc(&saved).expect("test parses saved");
    let endpoints = value["endpoints"].as_array().expect("test reads array");
    assert_eq!(endpoints[1]["name"], "studio");
    assert_eq!(endpoints[1]["kind"], "ollama");
    assert_eq!(endpoints[0]["name"], "local");
}
