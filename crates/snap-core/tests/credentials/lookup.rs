//! T3 자격 파일 이름 조회 시험. 정상 읽기·trim·이름 없음을 같은 책임으로 묶음.
use std::path::Path;

use snap_core::config::credentials_from_str;
use snap_core::config::find_api_key;

const PATH: &str = "dir/credentials.jsonc";

#[test]
fn reads_api_keys_table() {
    let text = "{ \"api_keys\": { \"primary\": \"primary-value\" } }";

    let keys = credentials_from_str(text, Path::new(PATH)).expect("test builds valid jsonc");

    let got = find_api_key(&keys, "primary").expect("test looks up existing name");
    assert_eq!(got.expose(), "primary-value");
}

#[test]
fn trims_name_on_both_sides() {
    let text = "{ \"api_keys\": { \"lab \": \"lab-value\" } }";

    let keys = credentials_from_str(text, Path::new(PATH)).expect("test builds valid jsonc");

    let got = find_api_key(&keys, " lab").expect("test looks up trimmed name");
    assert_eq!(got.expose(), "lab-value");
}

#[test]
fn missing_name_returns_none() {
    let text = "{ \"api_keys\": { \"primary\": \"primary-value\" } }";

    let keys = credentials_from_str(text, Path::new(PATH)).expect("test builds valid jsonc");

    assert!(find_api_key(&keys, "absent").is_none());
}
