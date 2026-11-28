//! 이름 규칙 검증 자리. trim한 값 기준 빈 값·`/`·중복 거부.

use serde_json::Value;

use crate::config::validate::error::ValidationError;
use crate::consts::limits::NAME_SEPARATOR;

/// 엔드포인트·모델 이름 규칙 확인.
pub fn check_names(root: &Value) -> Result<(), ValidationError> {
    let Some(endpoints) = root.get("endpoints").and_then(Value::as_array) else {
        return Ok(());
    };
    let mut used_endpoints: Vec<String> = Vec::new();
    for (index, endpoint) in endpoints.iter().enumerate() {
        check_endpoint(endpoint, index, &mut used_endpoints)?;
    }
    Ok(())
}

/// 엔드포인트 이름 하나와 그 아래 모델 이름 확인.
fn check_endpoint(
    endpoint: &Value,
    index: usize,
    used_endpoints: &mut Vec<String>,
) -> Result<(), ValidationError> {
    let Some(raw_name) = endpoint.get("name") else {
        return Err(ValidationError::EndpointNameMissing { index });
    };
    let Some(raw) = raw_name.as_str() else {
        // 문자열이 아닌 이름 값은 형식 오류로 보냄.
        return Ok(());
    };
    let name = raw.trim();
    if name.is_empty() {
        return Err(ValidationError::EmptyEndpointName { index });
    }
    if name.contains(NAME_SEPARATOR) {
        return Err(ValidationError::NameHasSeparator {
            name: name.to_owned(),
        });
    }
    if used_endpoints.iter().any(|used| used == name) {
        return Err(ValidationError::DuplicateEndpointName {
            endpoint: name.to_owned(),
        });
    }
    used_endpoints.push(name.to_owned());
    check_models(endpoint, name)
}

/// 같은 엔드포인트 안 모델 이름만 봄. 다른 엔드포인트의 같은 별칭은 허용.
fn check_models(endpoint: &Value, endpoint_name: &str) -> Result<(), ValidationError> {
    let Some(models) = endpoint.get("models").and_then(Value::as_array) else {
        return Ok(());
    };
    let mut used_models: Vec<String> = Vec::new();
    for model in models {
        let Some(raw_name) = model.get("name") else {
            return Err(ValidationError::ModelNameMissing {
                endpoint: endpoint_name.to_owned(),
            });
        };
        let Some(raw) = raw_name.as_str() else {
            // 문자열이 아닌 이름 값은 형식 오류로 보냄.
            continue;
        };
        let name = raw.trim();
        if name.is_empty() {
            return Err(ValidationError::EmptyModelName {
                endpoint: endpoint_name.to_owned(),
            });
        }
        if name.contains(NAME_SEPARATOR) {
            return Err(ValidationError::NameHasSeparator {
                name: name.to_owned(),
            });
        }
        check_model_id(model, endpoint_name, name)?;
        if used_models.iter().any(|used| used == name) {
            return Err(ValidationError::DuplicateModelName {
                endpoint: endpoint_name.to_owned(),
                model: name.to_owned(),
            });
        }
        used_models.push(name.to_owned());
    }
    Ok(())
}

/// 모델의 `id`가 trim 후 비면 거부.
fn check_model_id(
    model: &Value,
    endpoint_name: &str,
    model_name: &str,
) -> Result<(), ValidationError> {
    let Some(id) = model.get("id") else {
        return Ok(());
    };
    let is_empty = id.as_str().is_none_or(|id| id.trim().is_empty());
    if is_empty {
        return Err(ValidationError::EmptyModelId {
            endpoint: endpoint_name.to_owned(),
            model: model_name.to_owned(),
        });
    }
    Ok(())
}
