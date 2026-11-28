//! T4 자격 키 연산의 가리기 책임. 오류 문구·Debug에 키 값·파일 내용이 안 나오는지만 봄.
use snap_core::config::save::add_key;
use snap_core::config::save::remove_key;
use snap_core::config::save::update_key;

use crate::materials::write_credentials;

/// 키 값이 걸린 자격 파일 텍스트. 값이 형식 오류를 가진다.
const SECRET_TEXT: &str = "{\n  \"api_keys\": {\n    \"local\": \"SECRET-VALUE-6321\",\n    \"broken\": {\n      \"nested\": \"NESTED-VALUE-8890\"\n    }\n  }\n}";

#[test]
fn rejection_error_text_has_no_key_value() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), SECRET_TEXT);

    let error = add_key(&config_dir, "studio", "studio-value").expect_err("test expects rejection");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("SECRET-VALUE-6321"));
    assert!(!debug.contains("SECRET-VALUE-6321"));
    assert!(!display.contains("NESTED-VALUE-8890"));
    assert!(!debug.contains("NESTED-VALUE-8890"));
    assert!(!display.contains("studio-value"));
    assert!(!debug.contains("studio-value"));
}

#[test]
fn missing_target_error_text_has_no_key_value() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), SECRET_TEXT);

    let error =
        update_key(&config_dir, "absent", "ABSENT-VALUE-2455").expect_err("test expects rejection");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(display.contains("absent"));
    assert!(!display.contains("ABSENT-VALUE-2455"));
    assert!(!debug.contains("ABSENT-VALUE-2455"));
    assert!(!display.contains("SECRET-VALUE-6321"));
    assert!(!debug.contains("SECRET-VALUE-6321"));
}

#[test]
fn remove_rejection_error_text_has_no_key_value() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), SECRET_TEXT);

    let error = remove_key(&config_dir, "local").expect_err("test expects rejection");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("SECRET-VALUE-6321"));
    assert!(!debug.contains("SECRET-VALUE-6321"));
    assert!(!display.contains("NESTED-VALUE-8890"));
    assert!(!debug.contains("NESTED-VALUE-8890"));
}
