//! 모델 칸 `tool_call_form` 읽기입니다(D50 결정 7).
//!
//! 설정에 적는 이름(`openai`·`text`·`auto`)만 받아들이고, 그 밖의 값은 모델 이름만 밝히고 거부합니다.

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::{ConfigError, ValidationError};
use super::types::{DEFAULT_TOOL_CALL_FORM, ToolCallForm};

/// 모델의 `tool_call_form`을 읽습니다. 칸이 없으면 기본값(`openai`)이고, 값은 설정에 적는
/// 이름(`openai`·`text`·`auto`)만 받아들이며 그 밖의 값은 모델 이름만 밝히고 거부합니다(D50 결정 7).
pub(super) fn read_tool_call_form(
    model: &str,
    object: &mut JsonObject<'_>,
) -> Result<ToolCallForm, ConfigError> {
    let Some(value) = object.take("tool_call_form") else {
        return Ok(DEFAULT_TOOL_CALL_FORM);
    };
    let form = match &value {
        JsonValue::String(text) => ToolCallForm::parse(text),
        _ => None,
    };
    form.ok_or_else(|| ValidationError::InvalidToolCallForm {
        model: model.to_owned(),
    })
    .map_err(ConfigError::from)
}
