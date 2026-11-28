//! `http` 값 검증 자리. 키 7종과 backoff 범위를 봄.

use serde_json::Map;
use serde_json::Value;

use crate::config::validate::error::ValidationError;
use crate::config::validate::values::num::is_integer_at_least;
use crate::config::validate::values::num::is_seconds;
use crate::consts::limits::ALLOWED_HTTP_KEYS;
use crate::consts::limits::RETRIES_MIN;
use crate::locale::validate::HTTP_BLOCK;

/// `http` 키 7종과 값. 모르는 키는 `http.<항목>` 이름으로 거부.
pub(super) fn check_http(http: Option<&Value>) -> Result<(), ValidationError> {
    let Some(http) = http.filter(|value| !value.is_null()) else {
        return Ok(());
    };
    let Some(items) = http.as_object() else {
        return Err(ValidationError::HttpRejected {
            item: HTTP_BLOCK.to_owned(),
        });
    };
    check_http_items(items)?;
    check_backoff_range(items)
}

/// `http` 키 이름과 값 하나씩 확인.
fn check_http_items(items: &Map<String, Value>) -> Result<(), ValidationError> {
    for (key, value) in items {
        let is_known = ALLOWED_HTTP_KEYS.contains(&key.as_str());
        if !is_known {
            return Err(ValidationError::HttpRejected {
                item: format!("http.{key}"),
            });
        }
        let is_valid = match key.as_str() {
            "retries" => is_integer_at_least(value, RETRIES_MIN),
            _ => is_seconds(value),
        };
        if !is_valid {
            return Err(ValidationError::HttpRejected {
                item: format!("http.{key}"),
            });
        }
    }
    Ok(())
}

/// `backoff_min`이 `backoff_max`보다 크면 거부. 같은 값은 허용.
fn check_backoff_range(items: &Map<String, Value>) -> Result<(), ValidationError> {
    let min = items.get("backoff_min").and_then(Value::as_f64);
    let max = items.get("backoff_max").and_then(Value::as_f64);
    let is_too_big = min.is_some_and(|min| max.is_some_and(|max| min > max));
    if is_too_big {
        return Err(ValidationError::BackoffRange);
    }
    Ok(())
}
