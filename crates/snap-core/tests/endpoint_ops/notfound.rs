//! T2 대상 항목 없음 책임. 찾지 못하면 이름만 밝힌 오류이고 파일은 그대로인지 봄.
use snap_core::config::save::remove_endpoint;
use snap_core::config::save::update_endpoint;

use crate::materials::write_config;

/// 엔드포인트가 하나 있는 설정 텍스트.
const ONE_ENDPOINT: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    }\n  ]\n}";

#[test]
fn update_missing_name_reports_name_only() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), ONE_ENDPOINT);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let error = update_endpoint(&config_dir, "absent", "http://127.0.0.1:11434/v1", "ollama")
        .expect_err("test expects missing target");

    assert!(error.to_string().contains("absent"));
    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after missing target");
    assert_eq!(saved, original);
}

#[test]
fn remove_missing_name_reports_name_only() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), ONE_ENDPOINT);

    let error = remove_endpoint(&config_dir, "absent").expect_err("test expects missing target");

    assert!(error.to_string().contains("absent"));
}
