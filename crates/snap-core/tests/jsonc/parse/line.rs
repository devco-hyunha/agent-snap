//! T1 공용 JSONC 해석기 줄 번호 시험. 쉼표 누락·마지막 쉼표의 줄 번호만 밝힘을 봄.
use snap_core::jsonc::parse_jsonc;
use snap_core::locale::jsonc::JSONC_PARSE_FAILED;

// 해석기는 오류가 생긴 줄 번호를 1부터 시작. 값·줄 내용은 미포함.
#[test]
fn reports_line_of_missing_comma() {
    let text = "{\n  \"a\": 1\n  \"b\": 2\n}";

    let err = parse_jsonc(text).expect_err("test expects missing comma");
    let message = err.to_string();

    assert!(
        message.starts_with(JSONC_PARSE_FAILED),
        "message comes from locale key"
    );
    assert!(
        message.contains("line 2"),
        "message points at the line needing a comma"
    );
    assert!(
        !message.contains("\"b\""),
        "message hides the line content and value"
    );
}

#[test]
fn rejects_trailing_comma_with_line() {
    let text = "{\n  \"a\": 1,\n}";

    let err = parse_jsonc(text).expect_err("test expects trailing comma");
    let message = err.to_string();

    assert!(
        message.starts_with(JSONC_PARSE_FAILED),
        "message comes from locale key"
    );
    assert!(
        message.contains("line 2"),
        "message points at the trailing comma"
    );
    assert!(!message.contains("a\""), "message hides the key");
}
