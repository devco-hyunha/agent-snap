//! T3 모델 수정 시험: 수정 책임. 같은 엔드포인트 안 같은 이름 항목의 지정한 칸만 바뀌는지 봄(spec AC 3).
use std::path::Path;

use snap_core::config::config_from_str;
use snap_core::config::save::ModelChanges;
use snap_core::config::save::update_model;

use crate::materials::write_config;

/// 모델이 둘 있는 엔드포인트 설정 텍스트.
const TWO_MODELS: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\",\n          \"context_window\": 2048\n        },\n        {\n          \"name\": \"large\",\n          \"id\": \"large-id\",\n          \"context_window\": 4096\n        }\n      ]\n    }\n  ]\n}";

#[test]
fn update_changes_only_target_model_fields() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_MODELS);

    let changes = ModelChanges {
        id: Some("small-new".to_owned()),
        context_window: Some(16384),
        reasoning: None,
    };
    update_model(&config_dir, "local", "small", &changes).expect("test updates model");

    let config = config_from_str(
        &std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config"),
        Path::new("snap.jsonc"),
    )
    .expect("test reads updated model");
    let models = &config.endpoints[0].models;
    assert_eq!(models[0].id, "small-new");
    assert_eq!(models[0].context_window, 16384);
    assert_eq!(models[1].id, "large-id");
    assert_eq!(models[1].context_window, 4096);
}

#[test]
fn update_adds_missing_field_and_keeps_others() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_MODELS);

    let changes = ModelChanges {
        id: None,
        context_window: None,
        reasoning: Some("low".to_owned()),
    };
    update_model(&config_dir, "local", "small", &changes).expect("test updates model");

    let config = config_from_str(
        &std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config"),
        Path::new("snap.jsonc"),
    )
    .expect("test reads updated model");
    let models = &config.endpoints[0].models;
    assert_eq!(models[0].reasoning.as_deref(), Some("low"));
    assert_eq!(models[0].id, "small-id");
    assert_eq!(models[0].context_window, 2048);
    assert_eq!(models[1].reasoning, None);
}

#[test]
fn update_keeps_endpoint_fields() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_MODELS);

    let changes = ModelChanges {
        id: Some("small-new".to_owned()),
        context_window: None,
        reasoning: None,
    };
    update_model(&config_dir, "local", "small", &changes).expect("test updates model");

    let config = config_from_str(
        &std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config"),
        Path::new("snap.jsonc"),
    )
    .expect("test reads updated endpoint");
    let endpoint = &config.endpoints[0];
    assert_eq!(endpoint.name, "local");
    assert_eq!(endpoint.kind, "ollama");
    assert_eq!(endpoint.models.len(), 2);
}
