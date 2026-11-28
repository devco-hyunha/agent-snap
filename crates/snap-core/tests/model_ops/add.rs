//! T3 모델 추가 시험: 왕복 책임. 추가 뒤에 다시 읽혔는지와 `models` 칸 생성만 봄(spec AC 4).
use std::path::Path;

use snap_core::config::config_from_str;
use snap_core::config::save::add_model;

use crate::materials::write_config;

/// 모델이 하나 있는 엔드포인트 설정 텍스트.
const ONE_MODEL: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\"\n        }\n      ]\n    }\n  ]\n}";

#[test]
fn add_appends_model_and_read_shows_it() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), ONE_MODEL);

    add_model(&config_dir, "local", "large", "large-id", Some(8192)).expect("test adds model");

    let config = config_from_str(
        &std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config"),
        Path::new("snap.jsonc"),
    )
    .expect("test reads saved model");
    let models = &config.endpoints[0].models;
    assert_eq!(models.len(), 2);
    assert_eq!(models[1].name, "large");
    assert_eq!(models[1].id, "large-id");
    assert_eq!(models[1].context_window, 8192);
}

#[test]
fn add_without_context_window_uses_default() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), ONE_MODEL);

    add_model(&config_dir, "local", "large", "large-id", None).expect("test adds model");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(!saved.contains("context_window"));

    let config = config_from_str(&saved, Path::new("snap.jsonc")).expect("test reads saved model");
    let models = &config.endpoints[0].models;
    assert_eq!(
        models[1].context_window,
        snap_core::consts::defaults::DEFAULT_CONTEXT_WINDOW
    );
}

#[test]
fn add_creates_models_key_when_missing() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let no_models = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    }\n  ]\n}";
    let config_dir = write_config(base.path(), no_models);

    add_model(&config_dir, "local", "small", "small-id", None).expect("test adds model");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("\"models\""));
}
