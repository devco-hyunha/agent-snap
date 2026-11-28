//! T3 자격 파일 거부 시험. api_keys 누락·값 타입·파일 없음·값 비노출을 같은 책임으로 묶음.
use std::path::Path;

use snap_core::config::credentials_from_str;
use snap_core::config::read_credentials;
use snap_core::locale::credentials::CREDENTIALS_FILE_NOT_FOUND;
use snap_core::locale::credentials::CREDENTIALS_TYPE_FAILED;

const PATH: &str = "dir/credentials.jsonc";

#[test]
fn missing_api_keys_is_shape_error() {
    let text = "{ \"other\": { \"primary\": \"primary-value\" } }";

    let err =
        credentials_from_str(text, Path::new(PATH)).expect_err("test expects api_keys missing");
    let message = err.to_string();

    assert!(
        message.starts_with(CREDENTIALS_TYPE_FAILED),
        "message comes from locale key"
    );
    assert!(message.contains(PATH), "message carries path");
    assert!(!message.contains("primary-value"), "message hides value");
}

#[test]
fn non_string_value_is_shape_error() {
    let text = "{ \"api_keys\": { \"primary\": { \"nested\": \"nested-value\" } } }";

    let err =
        credentials_from_str(text, Path::new(PATH)).expect_err("test expects non-string value");
    let message = err.to_string();

    assert!(
        message.starts_with(CREDENTIALS_TYPE_FAILED),
        "message comes from locale key"
    );
    assert!(message.contains(PATH), "message carries path");
    assert!(!message.contains("nested-value"), "message hides value");
}

#[test]
fn absent_file_is_missing_file_error() {
    let dir = tempfile::tempdir().expect("test creates temp dir");

    let err = read_credentials(dir.path()).expect_err("test expects absent file");
    let message = err.to_string();

    assert!(
        message.starts_with(CREDENTIALS_FILE_NOT_FOUND),
        "message comes from locale key"
    );
    assert!(
        message.contains("credentials.jsonc"),
        "message carries file path"
    );
}
