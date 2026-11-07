//! CST(원문 구조 트리) 편집이 쓰는 저장 보조 함수: 설정 쪽·자격 쪽이 함께 쓰는 자리입니다.
//!
//! 이 파일의 책임은 하나입니다. 저장 파일의 CST 트리에서 trim한 이름으로 항목을 찾아 칸을
//! 바꾸거나 지우는 것. 같은 지식을 설정 쪽과 자격 쪽이 각자 적지 않게 모였습니다
//! (dev-principles 3절 DRY). 값 변환은 `save_values`가, 파일 읽기는 `save_read`가 따로
//! 맡습니다.

use jsonc_parser::cst::{CstArray, CstInputValue, CstNode, CstObject, CstObjectProp};

use super::super::read::validate::checked_name;

/// 배열 요소 중 trim한 `name` 칸 값이 `want`와 같은 요소를 찾습니다.
pub(super) fn find_named(elements: &[CstNode], want: &str) -> Option<CstNode> {
    elements
        .iter()
        .find(|element| named_element_is(element, want))
        .cloned()
}

/// 배열 요소 한 개의 `name` 칸 값(trim한 것)이 `want`와 같은지 봅니다.
///
/// 객체가 아닌 요소는 편집할 것이 없는 것과 같은 처리로 `false`입니다.
fn named_element_is(element: &CstNode, want: &str) -> bool {
    let raw = object_field_string(element.as_object(), "name").unwrap_or_default();
    checked_name(&raw).is_ok_and(|name| name == want)
}

/// `api_keys` 칸 중 이름이 같은 칸을 원문 순서대로 모습니다.
///
/// 02-1 해석이 칸 이름을 적힌 그대로 비교하므로, 자격 파일 쪽 찾기도 적힌 그대로 비교합니다
/// (D10). 설정 쪽 `find_named`가 trim한 이름을 쓰는 것과 비교 규칙이 다릅니다.
pub(super) fn properties_named(api_keys: &CstObject, name: &str) -> Vec<CstObjectProp> {
    api_keys
        .properties()
        .into_iter()
        .filter(|property| property.decoded_name().is_some_and(|found| found == name))
        .collect()
}

/// `api_keys` 칸 중 이름이 같은 칸을 골라 모두 지웁니다. 지운 칸 수를 돌려줍니다.
///
/// 이름이 겹치는 칸을 남기지 않는 규칙(02-1 왕복 규칙)을 자격 쪽 세 자리에서 함께 씁니다
/// (dev-principles 3절 DRY). 칸이 없으면 아무것도 지우지 않고 0을 돌려줍니다.
pub(super) fn remove_named(api_keys: &CstObject, name: &str) -> usize {
    remove_props(&properties_named(api_keys, name))
}

/// 칸 목록에서 칸을 모두 지웁니다. 지운 칸 수를 돌려줍니다.
pub(super) fn remove_props(properties: &[CstObjectProp]) -> usize {
    let count = properties.len();
    for property in properties {
        property.clone().remove();
    }
    count
}

/// 배열에 같은 이름 요소를 찾고, 없으면 빈 객체를 더합니다. 객체가 아닌 요소는 편집할 수
/// 없습니다.
pub(super) fn ensure_object(array: &CstArray, existing: Option<CstNode>) -> Option<CstObject> {
    let node = match existing {
        Some(node) => node,
        None => array.append(CstInputValue::Object(Vec::new())),
    };
    node.as_object()
}

/// 객체에 문자열 칸의 값을 바꾸고, 칸이 없으면 새로 더합니다.
pub(super) fn set_string(object: &CstObject, name: &str, value: &str) {
    set_optional(object, name, Some(CstInputValue::String(value.to_owned())));
}

/// 객체에 숫자 칸의 값을 바꾸고, 칸이 없으면 새로 더합니다.
pub(super) fn set_number(object: &CstObject, name: &str, value: &str) {
    set_optional(object, name, Some(CstInputValue::Number(value.to_owned())));
}

/// 객체의 칸 값을 새 값으로 바꾸고, 칸이 없으면 새로 더합니다.
///
/// 기본값을 파일에 강제로 쓰지 않는 규칙은 `None`을 넘기는 쪽이 정합니다(D14·D29).
pub(super) fn set_optional(object: &CstObject, name: &str, value: Option<CstInputValue>) {
    let Some(value) = value else { return };
    match object.get(name) {
        Some(prop) => prop.set_value(value),
        None => {
            object.append(name, value);
        }
    }
}

/// 객체 칸 값이 문자열이면 해석한 값을 꺼냅니다. 문자열이 아닌 칸은 `None`입니다.
pub(super) fn string_value_of(property: &CstObjectProp) -> Option<String> {
    string_of_node(&property.value()?)
}

/// 배열 요소 안의 객체 칸 값이 문자열이면 해석한 값을 꺼냅니다. 칸이 없거나 문자열이 아니면
/// `None`입니다.
fn object_field_string(object: Option<CstObject>, name: &str) -> Option<String> {
    let value = object?.get(name)?.value()?;
    string_of_node(&value)
}

/// CST 배열·칸 값(노드)이 문자열 리터럴이면 해석한 값을 꺼냅니다.
fn string_of_node(node: &CstNode) -> Option<String> {
    node.as_string_lit()
        .and_then(|literal| literal.decoded_value().ok())
}
