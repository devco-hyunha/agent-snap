//! T4 키 삭제 시험: 제거 책임. 같은 이름 항목만 사라지고 나머지는 그대로인지 봄.
use snap_core::config::find_api_key;
use snap_core::config::read_credentials;
use snap_core::config::save::add_key;
use snap_core::config::save::remove_key;
use snap_core::config::save::update_key;

use crate::materials::write_credentials;

/// 키가 둘 있는 자격 파일 텍스트.
const TWO_KEYS: &str = "{\n  \"api_keys\": {\n    \"local\": \"local-value\",\n    \"studio\": \"studio-value\"\n  }\n}";

#[test]
fn remove_drops_only_named_entry() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), TWO_KEYS);

    remove_key(&config_dir, "local").expect("test removes key");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    assert!(find_api_key(&keys, "local").is_none());
    let studio = find_api_key(&keys, "studio").expect("test looks up kept key");
    assert_eq!(studio.expose(), "studio-value");
}

#[test]
fn add_update_remove_roundtrip_keeps_table() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = write_credentials(base.path(), TWO_KEYS);

    add_key(&config_dir, "extra", "extra-value").expect("test adds key");
    update_key(&config_dir, "extra", "extra-new").expect("test updates key");
    remove_key(&config_dir, "extra").expect("test removes key");

    let keys = read_credentials(&config_dir).expect("test reads saved keys");
    assert_eq!(keys.len(), 2);
    assert!(find_api_key(&keys, "extra").is_none());
}
