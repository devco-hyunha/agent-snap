//! T2 설정 거부 시험: 구조 오류 책임. 값 타입이 틀린 경우만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn rejects_wrong_shape() {
    let text = r#"{ "endpoints": "primary" }"#;

    let err = config_from_str(text, Path::new(PATH)).expect_err("test expects shape error");
    let message = err.to_string();

    assert!(
        matches!(err, snap_core::config::ConfigError::InvalidShape { .. }),
        "shape error is its own kind"
    );
    assert!(message.contains(PATH), "message carries path");
    assert!(
        !message.contains("primary"),
        "message hides the input value"
    );
}
