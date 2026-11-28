//! api_keys 표에서 키 조회 자리.

use serde_json::Map;
use serde_json::Value;

use crate::config::secret_string::SecretString;

/// api_keys 표에서 trim한 엔드포인트 이름으로 키를 찾음. 없으면 None.
pub fn find_api_key(keys: &Map<String, Value>, endpoint: &str) -> Option<SecretString> {
    let target = endpoint.trim();
    let name = keys.keys().find(|name| name.trim() == target)?;
    let value = keys.get(name)?.as_str()?;
    Some(SecretString::new(value.to_owned()))
}
