//! T3 모델 연산의 대상 없음 책임. 엔드포인트·모델이 없으면 이름만 밝힌 오류이고 파일은 그대로인지 봄.
use snap_core::config::save::ModelChanges;
use snap_core::config::save::add_model;
use snap_core::config::save::remove_model;
use snap_core::config::save::update_model;

use crate::materials::write_config;

/// 모델이 하나 있는 엔드포인트 설정 텍스트.
const ONE_MODEL: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\"\n        }\n      ]\n    }\n  ]\n}";

#[test]
fn add_missing_endpoint_reports_name_only() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), ONE_MODEL);

    let error = add_model(&config_dir, "absent", "small", "small-id", None)
        .expect_err("test expects missing endpoint");

    assert!(error.to_string().contains("absent"));
}

#[test]
fn update_missing_model_reports_name_only() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), ONE_MODEL);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let changes = ModelChanges {
        id: Some("new-id".to_owned()),
        context_window: None,
        reasoning: None,
    };
    let error = update_model(&config_dir, "local", "absent", &changes)
        .expect_err("test expects missing model");

    assert!(error.to_string().contains("absent"));
    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after missing target");
    assert_eq!(saved, original);
}

#[test]
fn remove_missing_model_reports_name_only() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), ONE_MODEL);

    let error =
        remove_model(&config_dir, "local", "absent").expect_err("test expects missing model");

    assert!(error.to_string().contains("absent"));
}
