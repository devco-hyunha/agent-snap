//! T3 모델 연산의 주석 보존 책임. 사용자가 단 주석·빈 줄이 그대로 남는지 봄(spec AC 2).
use snap_core::config::save::ModelChanges;
use snap_core::config::save::add_model;
use snap_core::config::save::remove_model;
use snap_core::config::save::update_model;

use crate::materials::write_config;

/// 사용자가 단 주석과 빈 줄이 있는 설정 텍스트.
const COMMENTED: &str = "{\n  // 언어 지정\n  \"language\": \"ko\",\n\n  /* 엔드표 */\n  \"endpoints\": [\n    // 첫 항목\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": [\n        // 모델 표\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\"\n        }\n      ]\n    }\n  ]\n}";

#[test]
fn add_keeps_comments_and_blank_lines() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), COMMENTED);

    add_model(&config_dir, "local", "large", "large-id", None).expect("test adds model");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("// 언어 지정"));
    assert!(saved.contains("/* 엔드표 */"));
    assert!(saved.contains("// 첫 항목"));
    assert!(saved.contains("// 모델 표"));
    assert!(saved.contains("\n\n"));
    assert!(saved.contains("\"language\": \"ko\""));
}

#[test]
fn update_keeps_comments() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), COMMENTED);

    let changes = ModelChanges {
        id: Some("small-new".to_owned()),
        context_window: Some(4096),
        reasoning: None,
    };
    update_model(&config_dir, "local", "small", &changes).expect("test updates model");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("// 언어 지정"));
    assert!(saved.contains("/* 엔드표 */"));
    assert!(saved.contains("// 모델 표"));
}

#[test]
fn remove_keeps_comments() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), COMMENTED);

    remove_model(&config_dir, "local", "small").expect("test removes model");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("// 언어 지정"));
    assert!(saved.contains("/* 엔드표 */"));
    assert!(saved.contains("// 첫 항목"));
}
