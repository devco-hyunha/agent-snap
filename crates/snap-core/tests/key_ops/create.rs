//! T4 자격 파일 생성 책임. 자격 파일이 없을 때 최상위 `api_keys` 객체로 만들어지는지만 봄(spec AC 7).
use snap_core::config::find_api_key;
use snap_core::config::read_credentials;
use snap_core::config::save::add_key;
use snap_core::config::save::update_key;

#[test]
fn add_creates_credentials_file_with_api_keys() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = base.path().join("snap");

    add_key(&config_dir, "local", "local-value").expect("test adds key to missing file");

    let saved = std::fs::read_to_string(config_dir.join("credentials.jsonc"))
        .expect("test reads created credentials");
    assert!(saved.contains("\"api_keys\""));
    assert!(saved.contains("local-value"));
}

#[test]
fn created_file_reads_as_valid_credentials() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = base.path().join("snap");

    add_key(&config_dir, "local", "local-value").expect("test adds key to missing file");

    let keys = read_credentials(&config_dir).expect("test reads created credentials");
    let got = find_api_key(&keys, "local").expect("test looks up created key");
    assert_eq!(got.expose(), "local-value");
}

#[test]
fn update_creates_credentials_file_when_missing() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = base.path().join("snap");

    add_key(&config_dir, "local", "first-value").expect("test adds key to missing file");
    update_key(&config_dir, "local", "second-value").expect("test updates created key");

    let keys = read_credentials(&config_dir).expect("test reads updated credentials");
    let got = find_api_key(&keys, "local").expect("test looks up updated key");
    assert_eq!(got.expose(), "second-value");
    assert_eq!(keys.len(), 1);
}
