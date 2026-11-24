//! T2 설정 거부 시험: 해석 실패 책임. 줄 번호만 밝히는 해석 오류만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn parse_failure_reports_line_only() {
    let text = "{\n  \"language\": ko\n}";

    let err = config_from_str(text, Path::new(PATH)).expect_err("test expects bad token");
    let message = err.to_string();

    assert!(message.contains("line 2"), "message carries line number");
    assert!(!message.contains("ko"), "message hides line content");
}
