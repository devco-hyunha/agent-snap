//! T4 키 정하기 시험. 이름 있음·빈 문자열·이름 없음·자격 파일 없음의 네 경우.
use std::path::Path;

use serde_json::Map;
use serde_json::Value;

use snap_core::config::CredentialsError;
use snap_core::config::credentials_from_str;
use snap_core::config::read_credentials;
use snap_core::config::resolve_api_key;

const PATH: &str = "dir/credentials.jsonc";

/// 자격 파일 텍스트를 읽어 api_keys 표로 변환. 시험은 정상 경로만 세움.
fn keys(text: &str) -> Result<Map<String, Value>, CredentialsError> {
    credentials_from_str(text, Path::new(PATH))
}

#[test]
fn name_with_value_resolves_key() {
    let credentials = keys(r#"{ "api_keys": { "primary": "primary-value" } }"#);

    let got = resolve_api_key(credentials, "primary").expect("test resolves existing key");

    assert_eq!(got.expect("test expects a key").expose(), "primary-value");
}

#[test]
fn empty_value_resolves_none() {
    let credentials = keys(r#"{ "api_keys": { "primary": "" } }"#);

    let got = resolve_api_key(credentials, "primary").expect("test maps empty value");

    assert!(got.is_none());
}

#[test]
fn absent_name_resolves_none() {
    let credentials = keys(r#"{ "api_keys": { "primary": "primary-value" } }"#);

    let got = resolve_api_key(credentials, "absent").expect("test maps absent name");

    assert!(got.is_none());
}

#[test]
fn missing_file_resolves_none() {
    let dir = tempfile::tempdir().expect("test builds temp dir");
    let credentials = read_credentials(dir.path());

    let got = resolve_api_key(credentials, "primary").expect("test maps missing file");

    assert!(got.is_none());
}

#[test]
fn read_failure_passes_through() {
    let credentials = keys(r#"{ "api_keys": { "primary": } }"#);

    let resolved = resolve_api_key(credentials, "primary");

    assert!(resolved.is_err());
}
