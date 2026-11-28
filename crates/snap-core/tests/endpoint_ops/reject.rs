//! T2 저장 직전 검증 책임. 검증에 걸리는 값으로 부르면 저장이 거부되고 원본이 그대로인지 봄(spec AC 6).
use snap_core::config::save::add_endpoint;
use snap_core::config::save::update_endpoint;

use crate::materials::write_config;

/// 검증에 통과하는 설정 텍스트.
const VALID: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    }\n  ]\n}";

#[test]
fn add_rejects_bad_kind_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), VALID);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let error = add_endpoint(&config_dir, "broken", "http://127.0.0.1:9999/v1", "bogus")
        .expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, original);
    assert!(error.to_string().contains("kind"));
}

#[test]
fn update_rejects_bad_base_url_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), VALID);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let error = update_endpoint(&config_dir, "local", "http://127.0.0.1:11434/v1/", "ollama")
        .expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, original);
    assert!(error.to_string().contains("base_url"));
}

#[test]
fn add_rejects_key_field_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let with_key = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"api_key\": \"OLD-KEY-7431\"\n    }\n  ]\n}";
    let config_dir = write_config(base.path(), with_key);

    let error = add_endpoint(&config_dir, "extra", "http://127.0.0.1:8000/v1", "openai")
        .expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert!(saved.contains("OLD-KEY-7431"));
    assert!(!saved.contains("extra"));
    assert!(error.to_string().contains("credentials.jsonc"));
}

#[test]
fn add_rejects_non_array_endpoints_key_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), "{\n  \"endpoints\": \"not-array\"\n}");

    let error = add_endpoint(&config_dir, "local", "http://127.0.0.1:11434/v1", "ollama")
        .expect_err("test expects array rejection");

    assert!(error.to_string().contains("endpoints"));
    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, "{\n  \"endpoints\": \"not-array\"\n}");
}
