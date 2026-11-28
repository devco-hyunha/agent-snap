//! T2 엔드포인트 추가 시험: 왕복 책임. 추가 뒤에 다시 읽혔는지와 배열 생성만 봄(spec AC 1).
use std::path::Path;

use snap_core::config::config_from_str;
use snap_core::config::save::add_endpoint;

use crate::materials::write_config;

/// 엔드포인트가 하나도 없는 설정 텍스트.
const EMPTY_TEXT: &str = "{\n  \"language\": \"ko\"\n}";

#[test]
fn add_appends_endpoint_and_read_shows_it() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), EMPTY_TEXT);

    add_endpoint(&config_dir, "local", "http://127.0.0.1:11434/v1", "ollama")
        .expect("test adds endpoint");

    let config = config_from_str(
        &std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config"),
        Path::new("snap.jsonc"),
    )
    .expect("test reads saved endpoint");
    let endpoint = &config.endpoints[0];
    assert_eq!(endpoint.name, "local");
    assert_eq!(endpoint.kind, "ollama");
}

#[test]
fn add_creates_endpoints_key_when_missing() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), EMPTY_TEXT);

    add_endpoint(&config_dir, "local", "http://127.0.0.1:11434/v1", "ollama")
        .expect("test adds endpoint");

    let saved =
        std::fs::read_to_string(config_dir.join("snap.jsonc")).expect("test reads saved config");
    assert!(saved.contains("\"endpoints\""));
}
