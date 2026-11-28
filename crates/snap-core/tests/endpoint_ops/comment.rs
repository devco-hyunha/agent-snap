//! T2 엔드포인트 연산의 주석 보존 책임. 사용자가 단 주석·빈 줄이 그대로 남는지 봄(spec AC 2).
use snap_core::config::save::add_endpoint;
use snap_core::config::save::remove_endpoint;
use snap_core::config::save::update_endpoint;

use crate::materials::write_config;

/// 사용자가 단 주석과 빈 줄이 있는 설정 텍스트.
const COMMENTED: &str = "{\n  // 언어 지정\n  \"language\": \"ko\",\n\n  /* 엔드표 */\n  \"endpoints\": [\n    // 첫 항목\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    }\n  ]\n}";

/// 주석 두 개가 걸린 엔드포인트 두 개 텍스트. 삭제 후 둘째 주석 보존 확인용.
const TWO_COMMENTED: &str = "{\n  // 언어 지정\n  \"language\": \"ko\",\n  \"endpoints\": [\n    // 첫 항목\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    },\n    // 둘째 항목\n    {\n      \"name\": \"studio\",\n      \"base_url\": \"http://127.0.0.1:1234/v1\",\n      \"kind\": \"lmstudio\"\n    }\n  ]\n}";

#[test]
fn add_keeps_comments_and_blank_lines() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), COMMENTED);

    add_endpoint(
        &config_dir,
        "studio",
        "http://127.0.0.1:1234/v1",
        "lmstudio",
    )
    .expect("test adds endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("// 언어 지정"));
    assert!(saved.contains("/* 엔드표 */"));
    assert!(saved.contains("// 첫 항목"));
    assert!(saved.contains("\n\n"));
    assert!(saved.contains("\"language\": \"ko\""));
}

#[test]
fn update_keeps_comments() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), COMMENTED);

    update_endpoint(&config_dir, "local", "http://127.0.0.1:11435/v1", "ollama")
        .expect("test updates endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("// 언어 지정"));
    assert!(saved.contains("/* 엔드표 */"));
    assert!(saved.contains("// 첫 항목"));
}

#[test]
fn remove_keeps_comments() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), TWO_COMMENTED);

    remove_endpoint(&config_dir, "local").expect("test removes endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("// 언어 지정"));
    assert!(saved.contains("// 둘째 항목"));
    assert!(saved.contains("\"studio\""));
}
