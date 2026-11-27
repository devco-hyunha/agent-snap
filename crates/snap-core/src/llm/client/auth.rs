//! 인증 헤더 조립 자리. 헤더 값 만드는 한 책임만.

use crate::config::secret_string::SecretString;
use crate::consts::request::AUTH_HEADER;
use crate::consts::request::AUTH_SCHEME;

/// `Authorization` 헤더 이름과 값. 키가 없으면 헤더를 만들지 않음.
/// 빈 키 값은 설정 검증(`config/keys.rs`의 `api_key_or_none`)이 거름.
pub fn auth_header(key: Option<&SecretString>) -> Option<(String, String)> {
    let key = key?;
    let header_value = format!("{AUTH_SCHEME} {}", key.expose());
    Some((AUTH_HEADER.to_owned(), header_value))
}
