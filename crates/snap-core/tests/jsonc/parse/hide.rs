//! T1 공용 JSONC 해석기 가리기 시험. 모르는 토큰 줄의 내용을 숨김만 봄.
use snap_core::jsonc::parse_jsonc;

#[test]
fn hides_line_content_for_bad_token() {
    let text = "{\n  \"secret-key\": oops\n}";

    let err = parse_jsonc(text).expect_err("test expects bad token");
    let message = err.to_string();

    assert!(
        message.contains("line 2"),
        "message points at the bad token"
    );
    assert!(
        !message.contains("oops") && !message.contains("secret-key"),
        "message hides line content and key"
    );
}
