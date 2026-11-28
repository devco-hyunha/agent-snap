//! T3 자격 파일 jsonc 해석 시험. 주석 허용과 마지막 쉼표 거부를 같은 책임으로 묶음.
use std::path::Path;

use snap_core::config::credentials_from_str;
use snap_core::config::find_api_key;

const PATH: &str = "dir/credentials.jsonc";

#[test]
fn allows_comments_in_jsonc() {
    let text = "{\n  // comment\n  \"api_keys\": { \"primary\": \"primary-value\" }\n}";

    let keys = credentials_from_str(text, Path::new(PATH)).expect("test builds commented jsonc");

    let got = find_api_key(&keys, "primary").expect("test looks up existing name");
    assert_eq!(got.expose(), "primary-value");
}

#[test]
fn rejects_trailing_comma() {
    let text = "{ \"api_keys\": { \"primary\": \"primary-value\", } }";

    assert!(credentials_from_str(text, Path::new(PATH)).is_err());
}
