//! 값 규칙 검증 자리. `kind`·`reasoning`·`base_url`·`sampling`·`http`·`extra`·`language`·`context_window`를 봄.

mod endpoint;
mod http;
mod num;
mod sampling;

use serde_json::Value;

use crate::config::validate::error::ValidationError;
use crate::config::validate::values::endpoint::check_endpoints;
use crate::config::validate::values::http::check_http;
use crate::config::validate::values::sampling::check_sampling;
use crate::consts::defaults::ALLOWED_LANGUAGES;

/// 값 규칙 확인. 모르는 항목과 범위 밖 값을 이름만 밝히고 거부.
pub fn check_values(root: &Value) -> Result<(), ValidationError> {
    check_endpoints(root)?;
    check_sampling(root.get("sampling"), None)?;
    check_http(root.get("http"))?;
    check_language(root)?;
    Ok(())
}

/// `language` 값이 허용값 안에 있는지 확인. 입력 값은 출력하지 않음.
fn check_language(root: &Value) -> Result<(), ValidationError> {
    let Some(language) = root.get("language") else {
        return Ok(());
    };
    let Some(raw) = language.as_str() else {
        // 문자열이 아닌 언어 값은 형식 오류로 보냄.
        return Ok(());
    };
    let is_allowed = ALLOWED_LANGUAGES.contains(&raw);
    if is_allowed {
        return Ok(());
    }
    Err(ValidationError::Language)
}
