//! 모델 칸 `extra` 읽기입니다(D15).
//!
//! 예약 키가 하나라도 있으면 모델 이름만 밝히고 거부하고, 그 밖의 JSON 값은 하위 객체까지
//! [`ExtraValue`]로 그대로 담습니다.

use std::collections::BTreeMap;

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::{ConfigError, ModelIssue};
use super::types::ExtraValue;
use super::validate;
use crate::jsonc::{WrongType, pick_object, take_optional};

/// 모델의 `extra`를 읽습니다. 칸이 없으면 빈 맵이고, 예약 키가 있으면 모델 이름과 칸 이름을
/// 밝히고 거부합니다(D15).
pub(super) fn read_extra(
    endpoint: &str,
    model: &str,
    object: &mut JsonObject<'_>,
) -> Result<BTreeMap<String, ExtraValue>, ConfigError> {
    let Some(extra) = take_optional(object, "extra", pick_object)
        .map_err(|WrongType| ConfigError::invalid_model_field(endpoint, model, "extra"))?
    else {
        return Ok(BTreeMap::new());
    };
    if let Some(key) = validate::first_present_key(&extra, &validate::RESERVED_EXTRA_KEYS) {
        return Err(ModelIssue::ExtraReservedKey {
            model: model.to_owned(),
            key: key.to_owned(),
        }
        .into());
    }
    Ok(to_extra_map(extra))
}

fn to_extra_map(object: JsonObject<'_>) -> BTreeMap<String, ExtraValue> {
    object
        .into_iter()
        .map(|(key, value)| (key.into_owned(), to_extra_value(value)))
        .collect()
}

fn to_extra_value(value: JsonValue<'_>) -> ExtraValue {
    match value {
        JsonValue::Null => ExtraValue::Null,
        JsonValue::Boolean(flag) => ExtraValue::Bool(flag),
        JsonValue::Number(text) => ExtraValue::Number(text.to_owned()),
        JsonValue::String(text) => ExtraValue::String(text.into_owned()),
        JsonValue::Array(array) => {
            ExtraValue::Array(array.take_inner().into_iter().map(to_extra_value).collect())
        }
        JsonValue::Object(object) => ExtraValue::Object(to_extra_map(object)),
    }
}
