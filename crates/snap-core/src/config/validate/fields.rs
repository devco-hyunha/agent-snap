//! 거부 칸 검증 자리. 예전 형식 칸과 자격 키 칸을 이름만으로 거부.

use serde_json::Value;

use crate::config::validate::error::ValidationError;

/// 거부 칸 4종 확인. 값은 읽지도 출력하지 않음.
pub fn check_rejected_fields(root: &Value) -> Result<(), ValidationError> {
    check_legacy_models(root)?;
    let Some(endpoints) = root.get("endpoints").and_then(Value::as_array) else {
        return Ok(());
    };
    for endpoint in endpoints {
        check_endpoint_key_fields(endpoint)?;
        check_model_endpoint_fields(endpoint)?;
    }
    Ok(())
}

/// 엔드포인트의 `api_key`·`api_key_env` 칸. 키 값은 자격 파일에만 두는 자리.
fn check_endpoint_key_fields(endpoint: &Value) -> Result<(), ValidationError> {
    let Some(name) = endpoint.get("name").and_then(Value::as_str) else {
        return Ok(());
    };
    let has_key_field = KEY_FIELDS
        .iter()
        .any(|field| endpoint.get(*field).is_some());
    if has_key_field {
        return Err(ValidationError::EndpointKeyField {
            endpoint: name.trim().to_owned(),
        });
    }
    Ok(())
}

/// 모델의 옛 `endpoint` 칸. 엔드포인트 참조는 중첩 구조 기준.
fn check_model_endpoint_fields(endpoint: &Value) -> Result<(), ValidationError> {
    let Some(endpoint_name) = endpoint.get("name").and_then(Value::as_str) else {
        return Ok(());
    };
    let Some(models) = endpoint.get("models").and_then(Value::as_array) else {
        return Ok(());
    };
    for model in models {
        let has_endpoint_field = model.get("endpoint").is_some();
        if !has_endpoint_field {
            continue;
        }
        let model_name = model
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_owned();
        return Err(ValidationError::ModelEndpointField {
            endpoint: endpoint_name.trim().to_owned(),
            model: model_name,
        });
    }
    Ok(())
}

/// 최상위의 옛 `models` 칸. 모델은 `endpoints[].models`에 두는 구조.
fn check_legacy_models(root: &Value) -> Result<(), ValidationError> {
    let has_legacy_models = root.get(LEGACY_MODELS_KEY).is_some();
    if has_legacy_models {
        return Err(ValidationError::LegacyModels);
    }
    Ok(())
}

/// 거부하는 자격 키 칸 이름.
const KEY_FIELDS: [&str; 2] = ["api_key", "api_key_env"];

/// 최상위에서 거부하는 옛 칸 이름.
const LEGACY_MODELS_KEY: &str = "models";
