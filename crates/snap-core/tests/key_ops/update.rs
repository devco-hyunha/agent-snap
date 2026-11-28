//! T4 키 수정 시험: 대상 선택 책임. 같은 이름 항목의 값만 바뀌고 나머지는 그대로인지 봄(spec AC 3).
use snap_core::config::find_api_key;
use snap_core::config::read_credentials;
use snap_core::config::save::update_key;

use crate::materials::write_credentials;

/// 키가 둘 있는 자격 파일 텍스트.
const TWO_KEYS: &str = "{\n  \"api_keys\": {\n    \"local\": \"local-value\",\n    \"studio\": \"studio-value\"\n  }\n}";

#[test]
fn update_changes_only_named_value() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), TWO_KEYS);

    update_key(&config_dir, "local", "local-new").expect("test updates key");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    let local = find_api_key(&keys, "local").expect("test looks up updated key");
    let studio = find_api_key(&keys, "studio").expect("test looks up kept key");
    assert_eq!(local.expose(), "local-new");
    assert_eq!(studio.expose(), "studio-value");
}

#[test]
fn update_keeps_entry_count() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), TWO_KEYS);

    update_key(&config_dir, "studio", "studio-new").expect("test updates key");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    assert_eq!(keys.len(), 2);
}

#[test]
fn update_matches_trimmed_name() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let text = "{\n  \"api_keys\": {\n    \"local \": \"local-value\"\n  }\n}";
    let config_dir = write_credentials(base.path(), text);

    update_key(&config_dir, " local", "local-new").expect("test updates trimmed name");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    let got = find_api_key(&keys, "local").expect("test looks up trimmed name");
    assert_eq!(got.expose(), "local-new");
}
