//! 저장 직전 자격 구조 검사 자리. 규칙의 정본은 `api_keys_from_value`가 맡는다.

use serde_json::Value;
use std::path::Path;

use crate::config::credentials::api_keys_from_value;
use crate::config::save::error::ConfigSaveError;

/// 편집 결과를 기존 자격 구조 검사에 견줌. 실패는 저장을 거부한다.
pub(super) fn check_credentials(value: &Value, path: &Path) -> Result<(), ConfigSaveError> {
    api_keys_from_value(value, path)
        .map(|_| ())
        .map_err(|e| ConfigSaveError::Credentials(Box::new(e)))
}
