//! T1 CST 편집 창구 시험: 거부 책임. 최상위가 객체가 아니면 편집을 막는지만 봄.
#[test]
fn rejects_non_object_root() {
    let result = snap_core::jsonc::edit_jsonc("[1, 2]", |object| {
        let _ = object;
    });

    assert!(matches!(
        result,
        Err(snap_core::jsonc::JsoncError::RootNotObject)
    ));
}

#[test]
fn rejects_syntax_error_text() {
    let result = snap_core::jsonc::edit_jsonc("{ \"language\": }", |object| {
        let _ = object;
    });

    assert!(matches!(
        result,
        Err(snap_core::jsonc::JsoncError::Syntax { .. })
    ));
}
