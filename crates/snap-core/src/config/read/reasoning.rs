//! 모델 칸 `reasoning` 읽기입니다(D15).
//!
//! 설정에 적는 이름(`off`·`low`·`medium`·`high`)만 받아들이고, 모르는 값은 모델 이름만 밝히고 거부합니다.

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::{ConfigError, ModelIssue};
use super::types::Reasoning;

/// 모델의 `reasoning`을 읽습니다. 칸이 없으면 요청에 더하지 않는 `None`이고, 값은 설정에 적는
/// 이름만 받아들이며 그 밖의 값은 모델 이름만 밝히고 거부합니다.
pub(super) fn read_reasoning(
    model: &str,
    object: &mut JsonObject<'_>,
) -> Result<Option<Reasoning>, ConfigError> {
    let Some(value) = object.take("reasoning") else {
        return Ok(None);
    };
    let reasoning = match value {
        JsonValue::String(text) => Reasoning::parse(&text),
        _ => None,
    };
    reasoning
        .map(Some)
        .ok_or_else(|| ModelIssue::InvalidReasoning {
            model: model.to_owned(),
        })
        .map_err(ConfigError::from)
}
