//! `snap.jsonc`에서 항목을 고르는 규칙만 맡는 자리. 저장은 `save_validated`가 검증한다.
//! 배열이 없으면 만들고, 배열이 아니면 거부한다. 대상이 없으면 이름만 밝힌 오류를 낸다.

use jsonc_parser::cst::CstArray;
use jsonc_parser::cst::CstInputValue;
use jsonc_parser::cst::CstNode;
use jsonc_parser::cst::CstObject;
use std::path::Path;

use crate::config::save::error::ConfigSaveError;
use crate::config::save::errors::endpoint_not_found;
use crate::config::save::errors::endpoints_not_array;
use crate::config::save::errors::model_not_found;
use crate::config::save::errors::models_not_array;
use crate::config::save::keys::ENDPOINTS_KEY;
use crate::config::save::keys::MODELS_KEY;
use crate::config::save::keys::NAME_KEY;

/// `endpoints` 칸이 있는 최상위 객체에서 `endpoints` 배열을 얻음.
/// 칸이 없으면 빈 배열로 만들고, 배열이 아니면 경로만 밝히고 거부한다.
pub(super) fn endpoints_array(
    object: &CstObject,
    path: &Path,
) -> Result<CstArray, ConfigSaveError> {
    object
        .array_value_or_create(ENDPOINTS_KEY)
        .ok_or_else(|| endpoints_not_array(path))
}

/// trim한 이름으로 엔드포인트 항목을 찾음. 없으면 이름만 밝힌 오류.
pub(super) fn find_endpoint(array: &CstArray, name: &str) -> Result<CstNode, ConfigSaveError> {
    find_element(array, name).ok_or_else(|| endpoint_not_found(name))
}

/// trim한 이름으로 엔드포인트 항목을 객체로 찾음. 없으면 이름만 밝힌 오류.
pub(super) fn find_endpoint_object(
    array: &CstArray,
    name: &str,
) -> Result<CstObject, ConfigSaveError> {
    let target = find_endpoint(array, name)?;
    target.as_object().ok_or_else(|| endpoint_not_found(name))
}

/// 엔드포인트 항목 객체에서 `models` 배열을 얻음.
/// 칸이 없으면 빈 배열로 만들고, 배열이 아니면 엔드포인트 이름만 밝히고 거부한다.
pub(super) fn models_array(
    endpoint: &CstObject,
    endpoint_name: &str,
) -> Result<CstArray, ConfigSaveError> {
    endpoint
        .array_value_or_create(MODELS_KEY)
        .ok_or_else(|| models_not_array(endpoint_name))
}

/// trim한 이름으로 모델 항목을 찾음. 없으면 모델 이름만 밝힌 오류.
pub(super) fn find_model(array: &CstArray, name: &str) -> Result<CstNode, ConfigSaveError> {
    find_element(array, name).ok_or_else(|| model_not_found(name))
}

/// trim한 이름으로 모델 항목을 객체로 찾음. 없으면 모델 이름만 밝힌 오류.
pub(super) fn find_model_object(
    array: &CstArray,
    name: &str,
) -> Result<CstObject, ConfigSaveError> {
    let target = find_model(array, name)?;
    target.as_object().ok_or_else(|| model_not_found(name))
}

/// 항목의 칸을 같은 이름에서 값만 바꿈. 없는 칸은 끝에 더한다.
pub(super) fn replace_fields(object: &CstObject, fields: &[(String, CstInputValue)]) {
    for (key, value) in fields {
        let Some(prop) = object.get(key) else {
            object.append(key, value.clone());
            continue;
        };
        prop.replace_with(key, value.clone());
    }
}

/// 배열 안에서 trim한 이름과 같은 `name` 칸을 가진 항목을 고름.
fn find_element(array: &CstArray, name: &str) -> Option<CstNode> {
    let is_target = |element: &CstNode| element_name(element).is_some_and(|found| found == name);
    array.elements().into_iter().find(is_target)
}

/// 항목 안 `name` 값(trim). 엔드포인트·모델 둘 다 같은 규칙으로 고름.
fn element_name(element: &CstNode) -> Option<String> {
    element
        .as_object()?
        .get(NAME_KEY)?
        .value()?
        .as_string_lit()?
        .decoded_value()
        .ok()
        .map(|name| name.trim().to_owned())
}
