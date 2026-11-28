//! T3 모델 연산의 가리기 책임. 오류 문구·Debug에 키·주소·파일 내용이 안 나오는지만 봄.
use snap_core::config::save::ModelChanges;
use snap_core::config::save::add_model;
use snap_core::config::save::update_model;

use crate::materials::write_config;

/// 검증 규칙에 걸리는 주소와 키 값이 들어간 설정 텍스트.
const SECRET_TEXT: &str = "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1/\",\n      \"kind\": \"ollama\",\n      \"api_key\": \"SECRET-VALUE-4417\",\n      \"models\": [\n        {\n          \"name\": \"small\",\n          \"id\": \"small-id\",\n          \"context_window\": 0\n        }\n      ]\n    }\n  ]\n}";

#[test]
fn rejection_error_text_has_no_key_or_address() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), SECRET_TEXT);

    let changes = ModelChanges {
        id: Some("small-new".to_owned()),
        context_window: Some(4096),
        reasoning: None,
    };
    let error =
        update_model(&config_dir, "local", "small", &changes).expect_err("test expects rejection");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("SECRET-VALUE-4417"));
    assert!(!debug.contains("SECRET-VALUE-4417"));
    assert!(!display.contains("11434"));
    assert!(!debug.contains("11434"));
}

#[test]
fn add_rejection_error_text_has_no_key() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(base.path(), SECRET_TEXT);

    let error = add_model(&config_dir, "local", "extra", "extra-id", None)
        .expect_err("test expects rejection");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("SECRET-VALUE-4417"));
    assert!(!debug.contains("SECRET-VALUE-4417"));
}

#[test]
fn missing_target_error_text_has_no_address() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_config(
        base.path(),
        "{\n  \"language\": \"ko\",\n  \"endpoints\": [\n    {\n      \"name\": \"local\",\n      \"base_url\": \"http://127.0.0.1:11434/v1\",\n      \"kind\": \"ollama\",\n      \"models\": []\n    }\n  ]\n}",
    );

    let error = add_model(&config_dir, "absent", "small", "small-id", None)
        .expect_err("test expects missing endpoint");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("http://"));
    assert!(!debug.contains("http://"));
}
