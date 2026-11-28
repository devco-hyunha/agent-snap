//! T4 설정 파일 무변경 책임. 키 연산이 `snap.jsonc`를 건드리지 않는지만 봄(spec AC 5).
use snap_core::config::save::add_key;
use snap_core::config::save::remove_key;
use snap_core::config::save::update_key;

use crate::materials::write_credentials_and_config;

/// 키가 하나 있는 자격 파일 텍스트.
const ONE_KEY: &str = "{\n  \"api_keys\": {\n    \"local\": \"local-value\"\n  }\n}";

/// 엔드포인트가 하나 있는 설정 텍스트.
const ONE_ENDPOINT: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    }\n  ]\n}";

#[test]
fn key_ops_leave_config_file_unchanged() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials_and_config(base.path(), ONE_KEY, ONE_ENDPOINT);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    add_key(&config_dir, "studio", "studio-value").expect("test adds key");
    update_key(&config_dir, "studio", "studio-new").expect("test updates key");
    remove_key(&config_dir, "studio").expect("test removes key");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after key ops");
    assert_eq!(saved, original);
}

#[test]
fn key_value_never_shows_in_config_file() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials_and_config(base.path(), ONE_KEY, ONE_ENDPOINT);

    add_key(&config_dir, "local", "SECRET-VALUE-7713").expect("test adds key");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after adding key");
    assert!(!saved.contains("SECRET-VALUE-7713"));
    assert!(!saved.contains("api_keys"));
}
