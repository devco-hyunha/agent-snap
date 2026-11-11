//! 엔드포인트·모델 칸(`endpoints[]`와 그 안의 `models[]`) 읽기입니다(D10, D20).
//!
//! 이름·중복·거부 칸 확인은 `validate` 규칙을 따르고, 오류에는 이름만 담습니다.

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::{ConfigError, ValidationError};
use super::parse::read_unique_named;
use super::tool_call_form::read_tool_call_form;
use super::types::{EndpointConfig, EndpointKind, ModelConfig};
use super::{context_window, extra, reasoning, validate};
use crate::jsonc::{WrongType, pick_object, take_optional};
use crate::secrets::SecretValue;

/// 루트의 `endpoints`를 읽습니다. 생략하면 빈 목록입니다.
pub(super) fn read_endpoints(
    root: &mut JsonObject<'_>,
) -> Result<Vec<EndpointConfig>, ConfigError> {
    read_unique_named(
        root,
        "endpoints",
        || ConfigError::invalid_field("endpoints"),
        read_endpoint,
        |endpoint: &EndpointConfig| endpoint.name.as_str(),
        |endpoint| ValidationError::DuplicateEndpoint { endpoint }.into(),
    )
}

fn read_endpoint(index: usize, item: JsonValue<'_>) -> Result<EndpointConfig, ConfigError> {
    let JsonValue::Object(mut object) = item else {
        return Err(ConfigError::invalid_field(format!("endpoints[{index}]")));
    };
    let name = read_endpoint_name(index, &mut object)?;
    if validate::first_present_key(&object, &validate::ENDPOINT_KEY_FIELDS).is_some() {
        return Err(ValidationError::EndpointKeyField { endpoint: name }.into());
    }
    let base_url = object
        .take_string("base_url")
        .ok_or_else(|| ConfigError::invalid_endpoint_field(&name, "base_url"))?;
    if !validate::is_valid_base_url(&base_url) {
        return Err(ValidationError::InvalidBaseUrl { endpoint: name }.into());
    }
    let kind = object
        .take_string("kind")
        .ok_or_else(|| ConfigError::invalid_endpoint_field(&name, "kind"))?;
    let kind = EndpointKind::parse(&kind).ok_or_else(|| ValidationError::InvalidKind {
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
        validate::NameRejection::Empty => ValidationError::EmptyEndpointName.into(),
        validate::NameRejection::HasSlash { name } => {
            ValidationError::EndpointNameHasSlash { endpoint: name }.into()
        }
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
        |model| {
            ValidationError::DuplicateModel {
                endpoint: endpoint.to_owned(),
                model,
            }
            .into()
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
        return Err(ValidationError::ModelEndpointField { model: name }.into());
    }
    let id = read_model_id(endpoint, &name, &mut object)?;
    let reasoning = reasoning::read_reasoning(&name, &mut object)?;
    let sampling = take_optional(&mut object, "sampling", pick_object)
        .map_err(|WrongType| ConfigError::invalid_model_field(endpoint, &name, "sampling"))?
        .map(|sampling| super::sampling::read_sampling(sampling, &name))
        .transpose()?
        .unwrap_or_default();
    let extra = extra::read_extra(endpoint, &name, &mut object)?;
    let context_window = context_window::read_context_window(&name, &mut object)?;
    let tool_call_form = read_tool_call_form(&name, &mut object)?;
    Ok(ModelConfig {
        name,
        id,
        sampling,
        reasoning,
        extra,
        context_window,
        tool_call_form,
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
        validate::NameRejection::Empty => ValidationError::empty_model_name(endpoint).into(),
        validate::NameRejection::HasSlash { name } => ValidationError::ModelNameHasSlash {
            endpoint: endpoint.to_owned(),
            model: name,
        }
        .into(),
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
