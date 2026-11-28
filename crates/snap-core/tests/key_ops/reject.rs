//! T4 저장 직전 자격 구조 검사 책임. 검사에 걸리는 형식으로 부르면 저장이 거부되고 원본이 그대로인지 봄(spec AC 6).
use snap_core::config::save::add_key;
use snap_core::config::save::remove_key;
use snap_core::config::save::update_key;

use crate::materials::write_credentials;

/// 키 값이 숫자인 자격 파일 텍스트. 자격 구조 검사에 걸리는 형식.
const NUMBER_VALUE: &str =
    "{\n  \"api_keys\": {\n    \"local\": 7,\n    \"studio\": \"studio-value\"\n  }\n}";

#[test]
fn add_rejects_non_object_api_keys_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let not_object = "{\n  \"api_keys\": \"not-object\"\n}";
    let config_dir = write_credentials(base.path(), not_object);

    let error =
        add_key(&config_dir, "studio", "studio-value").expect_err("test expects object rejection");

    assert!(error.to_string().contains("api_keys"));
    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads credentials after rejection");
    assert_eq!(saved, not_object);
}

#[test]
fn add_rejects_non_string_value_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let nested =
        "{\n  \"api_keys\": {\n    \"local\": {\n      \"nested\": \"nested-value\"\n    }\n  }\n}";
    let config_dir = write_credentials(base.path(), nested);

    let error =
        add_key(&config_dir, "studio", "studio-value").expect_err("test expects shape rejection");

    assert!(error.to_string().contains("api_keys"));
    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads credentials after rejection");
    assert_eq!(saved, nested);
}

#[test]
fn update_rejects_non_string_value_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), NUMBER_VALUE);

    let error =
        update_key(&config_dir, "studio", "studio-new").expect_err("test expects shape rejection");

    assert!(error.to_string().contains("api_keys"));
    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads credentials after rejection");
    assert_eq!(saved, NUMBER_VALUE);
}

#[test]
fn remove_rejects_non_string_value_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), NUMBER_VALUE);

    let error = remove_key(&config_dir, "studio").expect_err("test expects shape rejection");

    assert!(error.to_string().contains("api_keys"));
    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads credentials after rejection");
    assert_eq!(saved, NUMBER_VALUE);
}

#[test]
fn add_rejects_broken_syntax_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let broken = "{\n  \"api_keys\": { \"local\": }";
    let config_dir = write_credentials(base.path(), broken);

    let error =
        add_key(&config_dir, "studio", "studio-value").expect_err("test expects syntax rejection");

    assert!(error.to_string().contains("line"));
    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads credentials after rejection");
    assert_eq!(saved, broken);
}
