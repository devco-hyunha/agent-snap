//! T1 CST 편집 창구 시험: 주석 보존 책임. 사용자가 단 주석·빈 줄이 그대로 남는지만 봄.
use snap_core::jsonc::CstInputValue;

#[test]
fn keeps_comments_and_blank_lines() {
    let text =
        "{\n  // 언어 지정\n  \"language\": \"ko\",\n\n  /* 엔드표 */\n  \"endpoints\": []\n}";

    let edited = snap_core::jsonc::edit_jsonc(text, |object| {
        object
            .get("language")
            .expect("test expects language key")
            .replace_with("language", CstInputValue::from("en"));
    })
    .expect("test edits commented jsonc");

    assert!(edited.contains("// 언어 지정"));
    assert!(edited.contains("/* 엔드표 */"));
    assert!(edited.contains("\n\n"));
    assert!(edited.contains("\"language\": \"en\""));
}
