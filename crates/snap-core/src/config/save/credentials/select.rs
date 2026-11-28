//! 자격 파일에서 항목을 고르는 규칙만 맡는 자리. 저장은 `save_validated`가 검증한다.
//! `api_keys`가 없으면 객체로 만들고, 객체가 아니면 거부한다. 대상이 없으면 이름만 밝힌 오류를 낸다.

use jsonc_parser::cst::CstObject;
use jsonc_parser::cst::CstObjectProp;
use std::path::Path;

use crate::config::save::error::ConfigSaveError;
use crate::config::save::errors::api_keys_not_object;
use crate::config::save::errors::endpoint_not_found;
use crate::consts::defaults::API_KEYS_KEY;

/// 최상위 객체에서 `api_keys` 객체를 얻음.
/// 칸이 없으면 빈 객체로 만들고, 객체가 아니면 경로만 밝히고 거부한다.
pub(super) fn api_keys_object(
    object: &CstObject,
    path: &Path,
) -> Result<CstObject, ConfigSaveError> {
    object
        .object_value_or_create(API_KEYS_KEY)
        .ok_or_else(|| api_keys_not_object(path))
}

/// trim한 엔드포인트 이름으로 키 항목을 찾음. 없으면 이름만 밝힌 오류.
pub(super) fn find_key(object: &CstObject, name: &str) -> Result<CstObjectProp, ConfigSaveError> {
    let target = name.trim();
    let found = object
        .properties()
        .into_iter()
        .find(|prop| prop.decoded_name().is_some_and(|key| key.trim() == target));
    found.ok_or_else(|| endpoint_not_found(name))
}
