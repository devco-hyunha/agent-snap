//! T2 엔드포인트 연산의 가리기 책임. 오류 문구·Debug에 키·주소·파일 내용이 안 나오는지만 봄.
use snap_core::config::save::add_endpoint;
use snap_core::config::save::update_endpoint;

use crate::materials::write_config;

/// 검증 규칙에 걸리는 주소와 키 값이 들어간 설정 텍스트.
const SECRET_TEXT: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1/\",\n      \"kind\": \"ollama\",\n      \"api_key\": \"SECRET-VALUE-4417\"\n    }\n  ]\n}";

#[test]
fn rejection_error_text_has_no_key_or_address() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), SECRET_TEXT);

    let error = update_endpoint(&config_dir, "local", "http://127.0.0.1:9999/v1", "ollama")
        .expect_err("test expects rejection");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("SECRET-VALUE-4417"));
    assert!(!debug.contains("SECRET-VALUE-4417"));
    assert!(!display.contains("11434"));
    assert!(!debug.contains("11434"));
    assert!(!display.contains("9999"));
    assert!(!debug.contains("9999"));
}

#[test]
fn add_rejection_error_text_has_no_key() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), SECRET_TEXT);

    let error = add_endpoint(&config_dir, "extra", "http://127.0.0.1:8000/v1", "openai")
        .expect_err("test expects rejection");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("SECRET-VALUE-4417"));
    assert!(!debug.contains("SECRET-VALUE-4417"));
    assert!(!display.contains("8000"));
    assert!(!debug.contains("8000"));
}

#[test]
fn missing_target_error_text_has_no_address() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(
        base.path(),
        "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\"\n    }\n  ]\n}",
    );

    let error = update_endpoint(&config_dir, "absent", "http://127.0.0.1:8888/v1", "ollama")
        .expect_err("test expects missing target");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("http://"));
    assert!(!debug.contains("http://"));
}
