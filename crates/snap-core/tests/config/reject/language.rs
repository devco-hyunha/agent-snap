//! T2 설정 거부 시험: 모르는 언어 책임. 언어 값 거부만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn rejects_unknown_language() {
    let text = r#"{ "language": "fr" }"#;

    let err = config_from_str(text, Path::new(PATH)).expect_err("test expects bad language");
    let message = err.to_string();

    assert!(message.contains("language"), "message names the option");
    assert!(!message.contains("fr"), "message hides the input value");
}
