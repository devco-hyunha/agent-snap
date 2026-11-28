//! T1 CST 편집 창구 시험: 값 교체 책임. 기존 키를 바꾸면 그 키만 바뀌는지만 봄.
use snap_core::jsonc::CstInputValue;

#[test]
fn replace_changes_only_target_key() {
    let text = "{\n  \"language\": \"ko\",\n  \"endpoints\": []\n}";

    let edited = snap_core::jsonc::edit_jsonc(text, |object| {
        object
            .get("language")
            .expect("test expects language key")
            .replace_with("language", CstInputValue::from("en"));
    })
    .expect("test edits jsonc");

    let value = snap_core::jsonc::parse_jsonc(&edited).expect("test parses edited jsonc");
    assert_eq!(value["language"], "en");
    assert_eq!(value["endpoints"], serde_json::json!([]));
}

#[test]
fn append_adds_key_at_end_and_keeps_others() {
    let text = "{\n  \"language\": \"ko\"\n}";

    let edited = snap_core::jsonc::edit_jsonc(text, |object| {
        object.append("endpoints", CstInputValue::Array(Vec::new()));
    })
    .expect("test appends key");

    let value = snap_core::jsonc::parse_jsonc(&edited).expect("test parses appended jsonc");
    assert_eq!(value["language"], "ko");
    assert_eq!(value["endpoints"], serde_json::json!([]));
}
