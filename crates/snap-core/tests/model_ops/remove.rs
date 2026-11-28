//! T3 모델 삭제 시험: 삭제 책임. 같은 엔드포인트 안 같은 이름 항목만 빠지는지 봄(spec AC 4).
use std::path::Path;

use snap_core::config::config_from_str;
use snap_core::config::save::remove_model;

use crate::materials::write_config;

/// 모델이 둘 있는 엔드포인트 설정 텍스트.
const TWO_MODELS: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\"\n        },\n        {\n          \"name\": \"large\",\n          \"id\": \"large-id\"\n        }\n      ]\n    }\n  ]\n}";

#[test]
fn remove_drops_only_target_model() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_MODELS);

    remove_model(&config_dir, "local", "small").expect("test removes model");

    let config = config_from_str(
        &std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config"),
        Path::new("snap.jsonc"),
    )
    .expect("test reads config after removal");
    let models = &config.endpoints[0].models;
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].name, "large");
}

#[test]
fn remove_keeps_endpoint_and_other_models_key() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_MODELS);

    remove_model(&config_dir, "local", "large").expect("test removes model");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("\"models\""));
    assert!(saved.contains("\"kind\": \"ollama\""));
    assert!(saved.contains("\"name\": \"local\""));
    assert!(!saved.contains("large-id"));
}
