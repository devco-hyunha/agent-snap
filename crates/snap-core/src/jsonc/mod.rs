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
mod tests;
