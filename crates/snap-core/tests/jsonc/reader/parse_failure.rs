//! T1 자격 파일 읽기의 해석 실패 전달 시험. 줄 번호만 밝히는 책임만 봄.
use std::path::Path;

use snap_core::config::credentials_from_str;
use snap_core::locale::config::CREDENTIALS_PARSE_FAILED;

const PATH: &str = "dir/credentials.jsonc";

#[test]
fn parse_failure_reports_line_only() {
    let text = "{\n  \"api_keys\": {\n  \"primary\": oops\n}";

    let err = credentials_from_str(text, Path::new(PATH)).expect_err("test expects bad token");
    let message = err.to_string();

    assert!(
        message.starts_with(CREDENTIALS_PARSE_FAILED),
        "message comes from locale key"
    );
    assert!(message.contains(PATH), "message carries path");
    assert!(message.contains("line 3"), "message carries line number");
    assert!(
        !message.contains("oops") && !message.contains("api_keys"),
        "message hides line content and key"
    );
}
