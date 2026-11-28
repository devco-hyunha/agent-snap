//! T4 자격 키 연산의 주석 보존 책임. 사용자가 단 주석·빈 줄이 그대로 남는지 봄(spec AC 2).
use snap_core::config::save::add_key;
use snap_core::config::save::remove_key;
use snap_core::config::save::update_key;

use crate::materials::write_credentials;

/// 사용자가 단 주석과 빈 줄이 있는 자격 파일 텍스트.
const COMMENTED: &str = "{\n  // 키 표\n\n  \"api_keys\": {\n    // 첫 키\n    \"local\": \"local-value\",\n    \"studio\": \"studio-value\"\n  }\n}";

#[test]
fn add_keeps_comments_and_blank_lines() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), COMMENTED);

    add_key(&config_dir, "extra", "extra-value").expect("test adds key");

    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads saved credentials");
    assert!(saved.contains("// 키 표"));
    assert!(saved.contains("// 첫 키"));
    assert!(saved.contains("\n\n"));
    assert!(saved.contains("\"api_keys\""));
}

#[test]
fn update_keeps_comments() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), COMMENTED);

    update_key(&config_dir, "local", "local-new").expect("test updates key");

    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads saved credentials");
    assert!(saved.contains("// 키 표"));
    assert!(saved.contains("// 첫 키"));
}

#[test]
fn remove_keeps_comments() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), COMMENTED);

    remove_key(&config_dir, "local").expect("test removes key");

    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads saved credentials");
    assert!(saved.contains("// 키 표"));
    assert!(saved.contains("\"studio\""));
}
