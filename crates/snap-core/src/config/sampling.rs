//! 샘플링 칸(`sampling`) 읽기입니다. 루트와 모델별 `sampling`이 같은 규칙을 씁니다(D14).
//!
//! 오류에는 항목 이름과 모델 이름만 담고 입력 값은 담지 않습니다.

use std::str::FromStr;

use jsonc_parser::{JsonObject, JsonValue};

use super::error::ConfigError;
use super::types::Sampling;
use super::validate;
use crate::jsonc::{WrongType, parse_number, pick_object, take_optional};

/// 루트 `sampling` 오류에서 모델 이름 자리에 쓰는 표기입니다.
const ROOT_SAMPLING_OWNER: &str = "기본";

/// 루트의 `sampling`을 읽습니다. 칸이 없으면 모든 항목이 비어 있는 기본값입니다.
pub(super) fn read_root_sampling(root: &mut JsonObject<'_>) -> Result<Sampling, ConfigError> {
    take_optional(root, "sampling", pick_object)
        .map_err(|WrongType| ConfigError::invalid_field("sampling"))?
        .map(|sampling| read_sampling(sampling, ROOT_SAMPLING_OWNER))
        .transpose()
        .map(Option::unwrap_or_default)
}

/// 샘플링 객체를 읽습니다. `owner`는 오류에 밝힐 모델 이름(루트는 "기본")입니다.
pub(super) fn read_sampling(object: JsonObject<'_>, owner: &str) -> Result<Sampling, ConfigError> {
    let mut sampling = Sampling::default();
    for (key, value) in object {
        let item = key.as_ref();
        let out_of_range = || ConfigError::SamplingOutOfRange {
            item: item.to_owned(),
            model: owner.to_owned(),
        };
        match item {
            "temperature" => {
                sampling.temperature =
                    read_checked(&value, validate::is_valid_temperature, &out_of_range)?;
            }
            "top_p" => {
                sampling.top_p = read_checked(&value, validate::is_valid_top_p, &out_of_range)?;
            }
            "max_tokens" => {
                sampling.max_tokens =
                    read_checked(&value, validate::is_valid_max_tokens, &out_of_range)?;
            }
            _ => {
                return Err(ConfigError::UnknownSamplingItem {
                    item: item.to_owned(),
                });
            }
        }
    }
    Ok(sampling)
}

/// 숫자를 `T`로 읽고 범위 규칙을 확인합니다. 읽지 못하거나 규칙을 어기면 `out_of_range` 오류입니다.
fn read_checked<T: FromStr>(
    value: &JsonValue<'_>,
    is_valid: fn(&T) -> bool,
    out_of_range: &dyn Fn() -> ConfigError,
) -> Result<Option<T>, ConfigError> {
    parse_number(value)
        .filter(is_valid)
        .map(Some)
        .ok_or_else(out_of_range)
}
