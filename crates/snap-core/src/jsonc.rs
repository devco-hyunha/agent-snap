//! 크레이트 내부 JSONC 공용 모듈입니다. 자격 파일과 설정 파일이 같은 해석 규칙을 쓰도록
//! 한 곳에 둡니다(DECISIONS D18, D21).
//!
//! JSONC 텍스트를 `ParseOptions::default()`로 해석하고, 문법 오류는 줄 번호만 담은 오류로
//! 바꾸며, 최상위가 객체인지 확인합니다. 오류는 입력 값을 담지 않으므로 호출한 쪽은 자기
//! 오류 타입으로 바꾸기만 하면 됩니다.

use std::str::FromStr;

use jsonc_parser::{JsonArray, JsonObject, JsonValue, ParseOptions};

/// JSONC 텍스트를 객체로 읽지 못했을 때의 오류입니다. 입력 값은 담지 않습니다.
#[derive(Debug)]
pub(crate) enum JsoncError {
    /// 문법 오류입니다. 줄 번호만 담고 줄 내용과 값은 담지 않습니다.
    Syntax { line: usize },
    /// 최상위가 객체가 아닙니다(빈 텍스트 포함).
    RootNotObject,
}

/// 칸이 있지만 기대한 JSON 타입이 아닙니다.
#[derive(Debug)]
pub(crate) struct WrongType;

/// JSONC 텍스트를 해석해 최상위 객체를 돌려줍니다.
///
/// 주석(`//`, `/* */`)과 마지막 쉼표를 허용합니다.
pub(crate) fn parse_root_object(text: &str) -> Result<JsonObject<'_>, JsoncError> {
    let root = jsonc_parser::parse_to_value(text, &ParseOptions::default()).map_err(|error| {
        JsoncError::Syntax {
            line: error.line_display(),
        }
    })?;
    match root {
        Some(JsonValue::Object(object)) => Ok(object),
        _ => Err(JsoncError::RootNotObject),
    }
}

/// 객체에서 칸을 꺼내 `pick`으로 기대한 타입으로 바꿉니다.
///
/// 칸이 없으면 `Ok(None)`, 있지만 타입이 맞지 않으면 [`WrongType`]입니다.
pub(crate) fn take_optional<'a, T>(
    owner: &mut JsonObject<'a>,
    field: &str,
    pick: fn(JsonValue<'a>) -> Option<T>,
) -> Result<Option<T>, WrongType> {
    match owner.take(field) {
        None => Ok(None),
        Some(value) => pick(value).map(Some).ok_or(WrongType),
    }
}

/// 값이 객체면 객체를 돌려줍니다.
pub(crate) fn pick_object(value: JsonValue<'_>) -> Option<JsonObject<'_>> {
    match value {
        JsonValue::Object(object) => Some(object),
        _ => None,
    }
}

/// 값이 배열이면 배열을 돌려줍니다.
pub(crate) fn pick_array(value: JsonValue<'_>) -> Option<JsonArray<'_>> {
    match value {
        JsonValue::Array(array) => Some(array),
        _ => None,
    }
}

/// 값이 숫자이고 `T`로 읽히면 그 값을 돌려줍니다.
pub(crate) fn parse_number<T: FromStr>(value: &JsonValue<'_>) -> Option<T> {
    match value {
        JsonValue::Number(text) => text.parse().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        JsoncError, WrongType, parse_number, parse_root_object, pick_array, pick_object,
        take_optional,
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
}
