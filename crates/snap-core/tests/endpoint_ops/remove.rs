//! T2 엔드포인트 삭제 시험: 제거 책임. 같은 이름 항목만 사라지는지, 왕복 뒤 배열이 유지되는지 봄.
use snap_core::config::save::add_endpoint;
use snap_core::config::save::remove_endpoint;
use snap_core::config::save::update_endpoint;

use crate::materials::write_config;

/// 엔드포인트 두 개가 있는 설정 텍스트.
const TWO_ENDPOINTS: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    },\n    {\n      \"name\": \"studio\",\n      \"base_url\": \"http://127.0.0.1:1234/v1\",\n      \"kind\": \"lmstudio\"\n    }\n  ]\n}";

#[test]
fn remove_drops_only_named_endpoint() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_ENDPOINTS);

    remove_endpoint(&config_dir, "local").expect("test removes endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    let value = snap_core::jsonc::parse_jsonc(&saved).expect("test parses saved");
    let endpoints = value["endpoints"].as_array().expect("test reads array");
    assert_eq!(endpoints.len(), 1);
    assert_eq!(endpoints[0]["name"], "studio");
}

#[test]
fn add_update_remove_roundtrip_keeps_list() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_ENDPOINTS);

    add_endpoint(&config_dir, "extra", "http://127.0.0.1:8000/v1", "openai")
        .expect("test adds endpoint");
    update_endpoint(&config_dir, "extra", "http://127.0.0.1:8001/v1", "openai")
        .expect("test updates endpoint");
    remove_endpoint(&config_dir, "extra").expect("test removes endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    let value = snap_core::jsonc::parse_jsonc(&saved).expect("test parses saved");
    let endpoints = value["endpoints"].as_array().expect("test reads array");
    assert_eq!(endpoints.len(), 2);
    assert_eq!(endpoints[0]["name"], "local");
    assert_eq!(endpoints[1]["name"], "studio");
}
