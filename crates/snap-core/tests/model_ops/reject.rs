//! T3 모델 연산의 저장 직전 검증 책임. 검증에 걸리는 값으로 부르면 저장이 거부되고 원본이 그대로인지 봄(spec AC 6).
use snap_core::config::save::ModelChanges;
use snap_core::config::save::add_model;
use snap_core::config::save::remove_model;
use snap_core::config::save::update_model;

use crate::materials::write_config;

/// 검증에 통과하는 설정 텍스트.
const VALID: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\",\n          \"context_window\": 2048\n        }\n      ]\n    }\n  ]\n}";

#[test]
fn add_rejects_zero_context_window_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), VALID);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let error = add_model(&config_dir, "local", "broken", "broken-id", Some(0))
        .expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, original);
    assert!(error.to_string().contains("context_window"));
}

#[test]
fn update_rejects_zero_context_window_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), VALID);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let changes = ModelChanges {
        id: None,
        context_window: Some(0),
        reasoning: None,
    };
    let error =
        update_model(&config_dir, "local", "small", &changes).expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, original);
    assert!(error.to_string().contains("context_window"));
}

#[test]
fn update_rejects_bad_reasoning_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), VALID);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let changes = ModelChanges {
        id: None,
        context_window: None,
        reasoning: Some("bogus".to_owned()),
    };
    let error =
        update_model(&config_dir, "local", "small", &changes).expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, original);
    assert!(error.to_string().contains("reasoning"));
}

#[test]
fn add_rejects_duplicate_model_name_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), VALID);
    let original =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads original config");

    let error = add_model(&config_dir, "local", "small", "other-id", None)
        .expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, original);
    assert!(error.to_string().contains("small"));
}

#[test]
fn add_rejects_non_array_models_key_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let models_not_array = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": \"not-array\"\n    }\n  ]\n}";
    let config_dir = write_config(base.path(), models_not_array);

    let error = add_model(&config_dir, "local", "small", "small-id", None)
        .expect_err("test expects array rejection");

    assert!(error.to_string().contains("models"));
    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, models_not_array);
}

#[test]
fn remove_rejects_bad_kind_in_result_and_keeps_original() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let bad_kind = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"bogus\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\"\n        }\n      ]\n    }\n  ]\n}";
    let config_dir = write_config(base.path(), bad_kind);

    let error = remove_model(&config_dir, "local", "small").expect_err("test expects rejection");

    let saved = std::fs::read_to_string(config_dir.join("snap.jsonc"))
        .expect("test reads config after rejection");
    assert_eq!(saved, bad_kind);
    assert!(error.to_string().contains("kind"));
}
