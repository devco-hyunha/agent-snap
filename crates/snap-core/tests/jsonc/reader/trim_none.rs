//! T1 자격 파일 읽기의 02-1 계승 시험. trim·None 규칙을 그대로 둠.
use std::path::Path;

use snap_core::config::credentials_from_str;
use snap_core::config::find_api_key;

const PATH: &str = "dir/credentials.jsonc";

#[test]
fn keeps_trim_and_none_rules() {
    let text = "{\n  // comment\n  \"api_keys\": { \"lab \": \"lab-value\" }\n}";

    let keys = credentials_from_str(text, Path::new(PATH)).expect("test builds commented jsonc");

    let got = find_api_key(&keys, " lab").expect("test looks up trimmed name");
    assert_eq!(got.expose(), "lab-value");
    assert!(find_api_key(&keys, "absent").is_none());
}
