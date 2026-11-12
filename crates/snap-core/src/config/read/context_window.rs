//! 모델 칸 `context_window` 읽기입니다(D29).

use jsonc_parser::JsonObject;

use super::super::error::{ConfigError, ModelIssue};
use super::types::DEFAULT_CONTEXT_WINDOW;
use super::validate;
use crate::jsonc::parse_number;

/// 모델의 `context_window`를 읽습니다. 칸이 없으면 65536 기본값이고, 값이 있으면 1 이상 정수만
/// 받아들이며 그 밖의 값은 모델 이름만 밝히고 거부합니다(D29).
pub(super) fn read_context_window(
    model: &str,
    object: &mut JsonObject<'_>,
) -> Result<u64, ConfigError> {
    let Some(value) = object.take("context_window") else {
        return Ok(DEFAULT_CONTEXT_WINDOW);
    };
    parse_number::<u64>(&value)
        .filter(validate::is_valid_context_window)
        .ok_or_else(|| ModelIssue::InvalidContextWindow {
            model: model.to_owned(),
        })
        .map_err(ConfigError::from)
}
