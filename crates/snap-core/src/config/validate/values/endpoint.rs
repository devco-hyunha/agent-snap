//! 엔드포인트 값 검증 자리. `kind`·`base_url`과 모델별 `reasoning`·`extra`를 봄.

use serde_json::Value;

use crate::config::validate::error::ValidationError;
use crate::config::validate::values::sampling::check_sampling;
use crate::consts::limits::ALLOWED_URL_SCHEMES;
use crate::consts::limits::RESERVED_EXTRA_KEYS;
use crate::llm::EndpointKind;
use crate::llm::ReasoningEffort;

/// 엔드포인트별 `kind`·`base_url`와 그 아래 모델 확인.
pub(super) fn check_endpoints(root: &Value) -> Result<(), ValidationError> {
    let Some(endpoints) = root.get("endpoints").and_then(Value::as_array) else {
        return Ok(());
    };
    for endpoint in endpoints {
        check_endpoint(endpoint)?;
    }
    Ok(())
}

/// 엔드포인트 하나를 `kind`·`base_url`과 모델별로 봄.
fn check_endpoint(endpoint: &Value) -> Result<(), ValidationError> {
    let name = endpoint_name(endpoint);
    check_kind(endpoint, &name)?;
    check_base_url(endpoint, &name)?;
    let Some(models) = endpoint.get("models").and_then(Value::as_array) else {
        return Ok(());
    };
    for model in models {
        let model_name = model
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_owned();
        check_reasoning(model, &model_name)?;
        check_extra(model, &model_name)?;
        check_sampling(model.get("sampling"), Some(&model_name))?;
    }
    Ok(())
}

/// 오류 문구에 쓸 엔드포인트 이름(trim 값).
fn endpoint_name(endpoint: &Value) -> String {
    endpoint
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_owned()
}

/// `kind`가 허용값 안에 있는지 봄. 허용 서버 형식의 한 정의는 `EndpointKind`.
fn check_kind(endpoint: &Value, endpoint_name: &str) -> Result<(), ValidationError> {
    let Some(kind) = endpoint.get("kind").and_then(Value::as_str) else {
        return Ok(());
    };
    let is_allowed = EndpointKind::from_setting(kind).is_some();
    if is_allowed {
        return Ok(());
    }
    Err(ValidationError::KindRejected {
        endpoint: endpoint_name.to_owned(),
    })
}

/// `base_url` 형식. scheme은 `http`/`https`만, 빈 값·앞뒤 공백·끝 슬래시 거부.
fn check_base_url(endpoint: &Value, endpoint_name: &str) -> Result<(), ValidationError> {
    let Some(raw) = endpoint.get("base_url").and_then(Value::as_str) else {
        return Ok(());
    };
    let has_space = raw != raw.trim();
    let has_slash_tail = raw.ends_with('/');
    let lower = raw.to_ascii_lowercase();
    let has_scheme = ALLOWED_URL_SCHEMES
        .iter()
        .any(|scheme| lower.starts_with(scheme));
    let is_valid = !raw.is_empty() && !has_space && !has_slash_tail && has_scheme;
    if is_valid {
        return Ok(());
    }
    Err(ValidationError::BaseUrlRejected {
        endpoint: endpoint_name.to_owned(),
    })
}

/// `reasoning`이 허용값 안에 있는지 봄. 허용 강도의 한 정의는 `ReasoningEffort`.
fn check_reasoning(model: &Value, model_name: &str) -> Result<(), ValidationError> {
    let Some(reasoning) = model.get("reasoning").and_then(Value::as_str) else {
        return Ok(());
    };
    let is_allowed = ReasoningEffort::from_setting(reasoning).is_some();
    if is_allowed {
        return Ok(());
    }
    Err(ValidationError::ReasoningRejected {
        model: model_name.to_owned(),
    })
}

/// `extra`에 예약 키가 있는지 봄. `extra` 값은 미포함.
fn check_extra(model: &Value, model_name: &str) -> Result<(), ValidationError> {
    let Some(extra) = model.get("extra").and_then(Value::as_object) else {
        return Ok(());
    };
    let Some(key) = RESERVED_EXTRA_KEYS
        .iter()
        .find(|key| extra.keys().any(|name| name.as_str() == **key))
    else {
        return Ok(());
    };
    Err(ValidationError::ExtraReservedKey {
        model: model_name.to_owned(),
        key: (*key).to_owned(),
    })
}
