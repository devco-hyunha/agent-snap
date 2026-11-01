//! JSONC 텍스트를 [`Config`]로 읽는 최상위 조립 모듈입니다. 칸 단위 검증 규칙은 `validate`
//! 모듈을, 샘플링과 http 읽기는 `sampling`·`http` 모듈을 씁니다. 오류 생성은 `error` 모듈이
//! 맡습니다.
//!
//! 오류에는 이름과 칸 이름만 담고 입력 값은 담지 않습니다. 거부하는 키 칸은 값을 읽지 않고
//! 칸이 있는지만 봅니다.

use std::collections::{BTreeMap, HashSet};

use jsonc_parser::{JsonArray, JsonObject, JsonValue};

use super::error::ConfigError;
use super::http::read_http;
use super::sampling::{read_root_sampling, read_sampling};
use super::types::{
    Config, DEFAULT_TARGET_LANGUAGE, EndpointConfig, EndpointKind, ExtraValue, ModelConfig,
    Reasoning,
};
use super::validate::{self, NameRejection};
use crate::jsonc::{self, WrongType, pick_array, pick_object, take_optional};
use crate::secrets::SecretValue;

impl Config {
    /// 설정 파일 텍스트를 JSONC로 읽어 검증합니다.
    ///
    /// 주석(`//`, `/* */`)과 마지막 쉼표를 허용하고, 모르는 칸은 무시합니다. 빠진 값은
    /// 기본값이 됩니다. 단, `sampling`·`http` 안의 모르는 항목과 거부 칸(엔드포인트의
    /// `api_key`·`api_key_env`, 최상위 `models`, 모델의 `endpoint`, `extra`의 예약 키)은
    /// 예외로 거부합니다.
    ///
    /// # 오류
    /// 문법 오류는 줄 번호만, 그 밖의 거부는 이름만 담은 [`ConfigError`]로 돌려줍니다.
    pub fn parse(text: &str) -> Result<Self, ConfigError> {
        let mut root = jsonc::parse_root_object(text)?;
        if root.get("models").is_some() {
            return Err(ConfigError::LegacyTopLevelModels);
        }
        Ok(Self {
            target_language: read_target_language(&mut root)?,
            endpoints: read_endpoints(&mut root)?,
            sampling: read_root_sampling(&mut root)?,
            http: read_http(&mut root)?,
        })
    }
}

fn read_target_language(root: &mut JsonObject<'_>) -> Result<String, ConfigError> {
    match root.take("target_language") {
        None => Ok(DEFAULT_TARGET_LANGUAGE.to_owned()),
        Some(JsonValue::String(language)) => Ok(language.into_owned()),
        Some(_) => Err(ConfigError::invalid_field("target_language")),
    }
}

/// 객체의 배열 칸을 꺼내 항목마다 `read_item`으로 읽고, trim한 이름이 겹치면 거부합니다.
///
/// 칸이 없으면 빈 목록입니다. `wrong_type`은 칸이 배열이 아닐 때, `duplicate_error`는 이름이
/// 겹칠 때 오류를 만듭니다.
fn read_unique_named<'a, T>(
    owner: &mut JsonObject<'a>,
    field: &str,
    wrong_type: impl FnOnce() -> ConfigError,
    mut read_item: impl FnMut(usize, JsonValue<'a>) -> Result<T, ConfigError>,
    name_of: fn(&T) -> &str,
    duplicate_error: impl Fn(String) -> ConfigError,
) -> Result<Vec<T>, ConfigError> {
    let items = take_optional(owner, field, pick_array)
        .map_err(|WrongType| wrong_type())?
        .map(JsonArray::take_inner)
        .unwrap_or_default();
    let mut seen_names = HashSet::new();
    let mut read_items = Vec::with_capacity(items.len());
    for (index, item) in items.into_iter().enumerate() {
        let read = read_item(index, item)?;
        if !seen_names.insert(name_of(&read).to_owned()) {
            return Err(duplicate_error(name_of(&read).to_owned()));
        }
        read_items.push(read);
    }
    Ok(read_items)
}

fn read_endpoints(root: &mut JsonObject<'_>) -> Result<Vec<EndpointConfig>, ConfigError> {
    read_unique_named(
        root,
        "endpoints",
        || ConfigError::invalid_field("endpoints"),
        read_endpoint,
        |endpoint: &EndpointConfig| endpoint.name.as_str(),
        |endpoint| ConfigError::DuplicateEndpoint { endpoint },
    )
}

fn read_endpoint(index: usize, item: JsonValue<'_>) -> Result<EndpointConfig, ConfigError> {
    let JsonValue::Object(mut object) = item else {
        return Err(ConfigError::invalid_field(format!("endpoints[{index}]")));
    };
    let name = read_endpoint_name(index, &mut object)?;
    if validate::first_present_key(&object, &validate::ENDPOINT_KEY_FIELDS).is_some() {
        return Err(ConfigError::EndpointKeyField { endpoint: name });
    }
    let base_url = object
        .take_string("base_url")
        .ok_or_else(|| ConfigError::invalid_endpoint_field(&name, "base_url"))?;
    if !validate::is_valid_base_url(&base_url) {
        return Err(ConfigError::InvalidBaseUrl { endpoint: name });
    }
    let kind = object
        .take_string("kind")
        .ok_or_else(|| ConfigError::invalid_endpoint_field(&name, "kind"))?;
    let kind = EndpointKind::parse(&kind).ok_or_else(|| ConfigError::InvalidKind {
        endpoint: name.clone(),
    })?;
    let models = read_models(&name, &mut object)?;
    Ok(EndpointConfig {
        name,
        base_url: SecretValue::new(base_url.into_owned()),
        kind,
        models,
    })
}

fn read_endpoint_name(index: usize, object: &mut JsonObject<'_>) -> Result<String, ConfigError> {
    let raw = object
        .take_string("name")
        .ok_or_else(|| ConfigError::invalid_field(format!("endpoints[{index}].name")))?;
    validate::checked_name(&raw).map_err(|rejection| match rejection {
        NameRejection::Empty => ConfigError::EmptyEndpointName,
        NameRejection::HasSlash { name } => ConfigError::EndpointNameHasSlash { endpoint: name },
    })
}

fn read_models(
    endpoint: &str,
    object: &mut JsonObject<'_>,
) -> Result<Vec<ModelConfig>, ConfigError> {
    read_unique_named(
        object,
        "models",
        || ConfigError::invalid_endpoint_field(endpoint, "models"),
        |index, item| read_model(endpoint, index, item),
        |model: &ModelConfig| model.name.as_str(),
        |model| ConfigError::DuplicateModel {
            endpoint: endpoint.to_owned(),
            model,
        },
    )
}

fn read_model(
    endpoint: &str,
    index: usize,
    item: JsonValue<'_>,
) -> Result<ModelConfig, ConfigError> {
    let JsonValue::Object(mut object) = item else {
        return Err(ConfigError::invalid_endpoint_field(
            endpoint,
            format!("models[{index}]"),
        ));
    };
    let name = read_model_name(endpoint, index, &mut object)?;
    if object.get("endpoint").is_some() {
        return Err(ConfigError::ModelEndpointField { model: name });
    }
    let id = read_model_id(endpoint, &name, &mut object)?;
    let reasoning = read_reasoning(&name, &mut object)?;
    let sampling = take_optional(&mut object, "sampling", pick_object)
        .map_err(|WrongType| ConfigError::invalid_model_field(endpoint, &name, "sampling"))?
        .map(|sampling| read_sampling(sampling, &name))
        .transpose()?
        .unwrap_or_default();
    let extra = read_extra(endpoint, &name, &mut object)?;
    Ok(ModelConfig {
        name,
        id,
        sampling,
        reasoning,
        extra,
    })
}

fn read_model_name(
    endpoint: &str,
    index: usize,
    object: &mut JsonObject<'_>,
) -> Result<String, ConfigError> {
    let raw = object.take_string("name").ok_or_else(|| {
        ConfigError::invalid_endpoint_field(endpoint, format!("models[{index}].name"))
    })?;
    validate::checked_name(&raw).map_err(|rejection| match rejection {
        NameRejection::Empty => ConfigError::empty_model_name(endpoint),
        NameRejection::HasSlash { name } => ConfigError::ModelNameHasSlash {
            endpoint: endpoint.to_owned(),
            model: name,
        },
    })
}

fn read_model_id(
    endpoint: &str,
    model: &str,
    object: &mut JsonObject<'_>,
) -> Result<String, ConfigError> {
    let raw = object
        .take_string("id")
        .ok_or_else(|| ConfigError::invalid_model_field(endpoint, model, "id"))?;
    validate::trimmed_non_empty(&raw).ok_or_else(|| ConfigError::empty_model_id(endpoint, model))
}

fn read_reasoning(
    model: &str,
    object: &mut JsonObject<'_>,
) -> Result<Option<Reasoning>, ConfigError> {
    let Some(value) = object.take("reasoning") else {
        return Ok(None);
    };
    let reasoning = match value {
        JsonValue::String(text) => Reasoning::parse(&text),
        _ => None,
    };
    reasoning
        .map(Some)
        .ok_or_else(|| ConfigError::InvalidReasoning {
            model: model.to_owned(),
        })
}

fn read_extra(
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
        return Err(ConfigError::ExtraReservedKey {
            model: model.to_owned(),
            key: key.to_owned(),
        });
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
