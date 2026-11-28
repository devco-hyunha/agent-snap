//! 예시 파일 시험: `context_window` 안내 주석 책임. 주석 내용과 값 미포함만 봄.
use snap_core::config::config_from_str;
use snap_core::consts::defaults::DEFAULT_CONTEXT_WINDOW;

use crate::paths::example_path;

#[test]
fn example_file_documents_context_window() {
    let text = std::fs::read_to_string(example_path()).expect("test reads example file");

    // 안내 주석이 값 규칙과 기본값 숫자를 함께 밝힘.
    assert!(
        text.contains("context_window") && text.contains("1 이상 정수") && text.contains("65536"),
        "example file has context_window guide comment"
    );
}

#[test]
fn example_file_keeps_context_window_unset() {
    let path = example_path();
    let text = std::fs::read_to_string(&path).expect("test reads example file");

    // 주석만 두고 실제 키 칸은 두지 않음. 값은 기본값으로 정규화됨.
    assert!(
        !text.contains("\"context_window\":"),
        "example file has no context_window value"
    );
    let config = config_from_str(&text, &path).expect("test expects example file to pass");
    assert_eq!(
        config.endpoints[0].models[0].context_window, DEFAULT_CONTEXT_WINDOW,
        "unset context_window normalizes to default"
    );
}
