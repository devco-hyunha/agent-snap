//! 루트 칸 `target_language` 읽기입니다.
//!
//! 칸이 없으면 기본값(`ko`)이고, 문자열이 아니면 칸 이름만 밝히고 거부합니다.

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::ConfigError;
use super::types::DEFAULT_TARGET_LANGUAGE;

/// 루트의 `target_language`를 읽습니다.
pub(super) fn read_target_language(root: &mut JsonObject<'_>) -> Result<String, ConfigError> {
    match root.take("target_language") {
        None => Ok(DEFAULT_TARGET_LANGUAGE.to_owned()),
        Some(JsonValue::String(language)) => Ok(language.into_owned()),
        Some(_) => Err(ConfigError::invalid_field("target_language")),
    }
}
