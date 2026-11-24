//! 엔드포인트별 키 정하기 자리. 02-1의 이름 조회 재사용.

use serde_json::Map;
use serde_json::Value;

use crate::config::credentials::CredentialsError;
use crate::config::credentials::find_api_key;
use crate::config::secret_string::SecretString;

/// 자격 파일 읽기 결과와 엔드포인트 이름으로 정하는 키.
/// 자격 파일이 없는 경우만 키 없음, 다른 실패는 그대로 전달.
pub fn resolve_api_key(
    credentials: Result<Map<String, Value>, CredentialsError>,
    endpoint: &str,
) -> Result<Option<SecretString>, CredentialsError> {
    let keys = match credentials {
        Ok(keys) => keys,
        Err(CredentialsError::MissingFile { .. }) => return Ok(None),
        Err(error) => return Err(error),
    };
    Ok(api_key_or_none(&keys, endpoint))
}

/// trim한 이름의 키 값. 빈 문자열이면 키 없음.
fn api_key_or_none(keys: &Map<String, Value>, endpoint: &str) -> Option<SecretString> {
    let key = find_api_key(keys, endpoint)?;
    has_value(&key).then_some(key)
}

/// 키 값이 빈 문자열이 아닌지.
fn has_value(key: &SecretString) -> bool {
    !key.expose().is_empty()
}
