//! api_keys 구조 검증 자리.

use std::path::Path;

use serde_json::Map;
use serde_json::Value;

use crate::config::credentials::error::CredentialsError;
use crate::consts::defaults::API_KEYS_KEY;

/// 값에서 api_keys 표만 꺼냄. 누락·비객체·비문자열 값은 검증 실패로 잡음.
pub(super) fn api_keys_from_value(
    value: &Value,
    path_display: &Path,
) -> Result<Map<String, Value>, CredentialsError> {
    let root = value
        .as_object()
        .ok_or_else(|| invalid_shape(path_display))?;
    let keys = root
        .get(API_KEYS_KEY)
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_shape(path_display))?;
    if keys.values().any(|item| item.as_str().is_none()) {
        return Err(invalid_shape(path_display));
    }
    Ok(keys.clone())
}

/// 검증 실패를 경로와 함께 만드는 자리.
fn invalid_shape(path_display: &Path) -> CredentialsError {
    CredentialsError::InvalidShape {
        path: path_display.display().to_string(),
    }
}
