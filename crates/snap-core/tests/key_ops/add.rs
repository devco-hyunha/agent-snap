//! T4 키 추가 시험: 왕복 책임. 자격 파일에 키가 쓰이고 다시 읽히는지만 봄(spec AC 5).
use snap_core::config::find_api_key;
use snap_core::config::read_credentials;
use snap_core::config::save::add_key;

use crate::materials::write_credentials;

/// 키가 하나 있는 자격 파일 텍스트.
const ONE_KEY: &str = "{\n  \"api_keys\": {\n    \"local\": \"local-value\"\n  }\n}";

#[test]
fn add_appends_key_and_read_shows_it() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), ONE_KEY);

    add_key(&config_dir, "studio", "studio-value").expect("test adds key");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    let got = find_api_key(&keys, "studio").expect("test looks up added key");
    assert_eq!(got.expose(), "studio-value");
}

#[test]
fn add_keeps_other_entries() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), ONE_KEY);

    add_key(&config_dir, "studio", "studio-value").expect("test adds key");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    let local = find_api_key(&keys, "local").expect("test looks up kept key");
    assert_eq!(local.expose(), "local-value");
    assert_eq!(keys.len(), 2);
}

#[test]
fn add_trims_name_and_value() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), ONE_KEY);

    add_key(&config_dir, " studio ", " studio-value ").expect("test adds trimmed key");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    let got = find_api_key(&keys, "studio").expect("test looks up trimmed key");
    assert_eq!(got.expose(), "studio-value");
}
