//! T4 대상 항목 없음 책임. 자격 키 표에 그 이름이 없으면 이름만 밝힌 오류이고 파일은 그대로인지 봄.
use snap_core::config::save::remove_key;
use snap_core::config::save::update_key;

use crate::materials::write_credentials;

/// 키가 하나 있는 자격 파일 텍스트.
const ONE_KEY: &str = "{\n  \"api_keys\": {\n    \"local\": \"local-value\"\n  }\n}";

#[test]
fn update_missing_name_reports_name_only() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), ONE_KEY);
    let original = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads original credentials");

    let error =
        update_key(&config_dir, "absent", "absent-value").expect_err("test expects missing target");

    assert!(error.to_string().contains("absent"));
    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads credentials after missing target");
    assert_eq!(saved, original);
}

#[test]
fn remove_missing_name_reports_name_only() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), ONE_KEY);
    let original = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads original credentials");

    let error = remove_key(&config_dir, "absent").expect_err("test expects missing target");

    assert!(error.to_string().contains("absent"));
    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads credentials after missing target");
    assert_eq!(saved, original);
}
