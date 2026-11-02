use super::{
    JsoncError, WrongType, parse_number, parse_root_object, pick_array, pick_object, take_optional,
};

/// 주석과 마지막 쉼표가 든 텍스트에서 최상위 객체를 읽는다.
#[test]
fn parse_root_object_accepts_comments_and_trailing_comma() {
    let text = "{ // 주석\n \"a\": 1, /* 주석 */ }";

    let mut root = parse_root_object(text).expect("주석과 마지막 쉼표를 허용해야 합니다");

    assert!(root.take("a").is_some());
}

/// 문법 오류는 줄 번호만 담고 줄 내용과 값은 담지 않는다.
#[test]
fn parse_root_object_syntax_error_carries_line_number_only() {
    let text = "{ \"key\": \"secret-value-777\" }\n] }";

    let error = parse_root_object(text).expect_err("문법 오류여야 합니다");

    assert!(matches!(error, JsoncError::Syntax { line: 2 }));
    let debug = format!("{error:?}");
    assert!(!debug.contains("secret-value-777"));
    assert!(!debug.contains("key"));
}

/// 빈 텍스트와 객체가 아닌 최상위는 값 없는 오류로 거부한다.
#[test]
fn parse_root_object_rejects_empty_and_non_object_root() {
    for text in ["", "   ", "[]", r#""secret-value-777""#, "7", "null"] {
        let error = parse_root_object(text).expect_err("거부되어야 합니다");

        assert!(matches!(error, JsoncError::RootNotObject), "{text:?}");
        assert!(!format!("{error:?}").contains("secret-value-777"));
    }
}

/// 칸이 없으면 `None`, 타입이 맞으면 값, 타입이 다르면 `WrongType`이다.
#[test]
fn take_optional_separates_missing_matching_and_wrong_type() {
    let mut root = parse_root_object(r#"{"object":{},"array":[],"number":3}"#)
        .expect("정상 텍스트여야 합니다");

    assert!(matches!(
        take_optional(&mut root, "absent", pick_object),
        Ok(None)
    ));
    assert!(matches!(
        take_optional(&mut root, "object", pick_object),
        Ok(Some(_))
    ));
    assert!(matches!(
        take_optional(&mut root, "array", pick_array),
        Ok(Some(_))
    ));
    assert!(matches!(
        take_optional(&mut root, "number", pick_object),
        Err(WrongType)
    ));
}

/// 꺼낸 칸은 객체에서 빠져 두 번째로 꺼내면 없는 칸이 된다.
#[test]
fn take_optional_consumes_the_field() {
    let mut root = parse_root_object(r#"{"object":{}}"#).expect("정상 텍스트여야 합니다");

    assert!(matches!(
        take_optional(&mut root, "object", pick_object),
        Ok(Some(_))
    ));
    assert!(matches!(
        take_optional(&mut root, "object", pick_object),
        Ok(None)
    ));
}

/// 객체와 배열 도우미는 다른 타입의 값을 거부한다.
#[test]
fn pick_helpers_reject_other_json_types() {
    let mut root =
        parse_root_object(r#"{"object":{},"array":[]}"#).expect("정상 텍스트여야 합니다");
    let object = root.take("object").expect("object 칸");
    let array = root.take("array").expect("array 칸");

    assert!(pick_array(object).is_none());
    assert!(pick_object(array).is_none());
}

/// 숫자만 읽히고 문자열·타입 불일치(정수에 소수)는 `None`이다.
#[test]
fn parse_number_reads_numbers_of_the_requested_type_only() {
    let mut root = parse_root_object(r#"{"int":12,"fraction":1.5,"text":"12"}"#)
        .expect("정상 텍스트여야 합니다");
    let int = root.take("int").expect("int 칸");
    let fraction = root.take("fraction").expect("fraction 칸");
    let text = root.take("text").expect("text 칸");

    assert_eq!(parse_number::<u64>(&int), Some(12));
    assert_eq!(parse_number::<f64>(&fraction), Some(1.5));
    assert_eq!(parse_number::<u64>(&fraction), None);
    assert_eq!(parse_number::<u64>(&text), None);
}
