//! T3 엔드포인트 보존 책임. 모델 연산이 `endpoints` 배열 항목은 건드리지 않는지만 봄(spec AC 4).
use std::path::Path;

use snap_core::config::config_from_str;
use snap_core::config::save::ModelChanges;
use snap_core::config::save::add_model;
use snap_core::config::save::remove_model;
use snap_core::config::save::update_model;

use crate::materials::write_config;

/// 엔드포인트가 둘 있는 설정 텍스트.
const TWO_ENDPOINTS: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\"\n        }\n      ]\n    },\n    {\n      \"name\": \"studio\",\n      \"base_url\": \"http://127.0.0.1:1234/v1\",\n      \"kind\": \"lmstudio\"\n    }\n  ]\n}";

#[test]
fn model_ops_keep_endpoint_count_and_order() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_ENDPOINTS);

    add_model(&config_dir, "local", "large", "large-id", None).expect("test adds model");
    let changes = ModelChanges {
        id: Some("small-new".to_owned()),
        context_window: None,
        reasoning: None,
    };
    update_model(&config_dir, "local", "small", &changes).expect("test updates model");
    remove_model(&config_dir, "local", "small").expect("test removes model");

    let config = config_from_str(
        &std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config"),
        Path::new("snap.jsonc"),
    )
    .expect("test reads config after model ops");
    assert_eq!(config.endpoints.len(), 2);
    assert_eq!(config.endpoints[0].name, "local");
    assert_eq!(config.endpoints[1].name, "studio");
    assert_eq!(config.endpoints[1].kind, "lmstudio");
    assert!(config.endpoints[1].models.is_empty());
}
