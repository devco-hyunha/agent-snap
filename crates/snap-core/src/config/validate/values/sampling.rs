//! 샘플링 값 검증 자리. 항목 이름과 범위를 봄.

use serde_json::Map;
use serde_json::Value;

use crate::config::validate::error::ValidationError;
use crate::config::validate::values::num::is_integer_at_least;
use crate::config::validate::values::num::is_number_between;
use crate::consts::limits::MAX_TOKENS_MIN;
use crate::consts::limits::SAMPLING_ITEMS;
use crate::consts::limits::TEMPERATURE_MAX;
use crate::consts::limits::TEMPERATURE_MIN;
use crate::consts::limits::TOP_P_MAX;
use crate::consts::limits::TOP_P_MIN_EXCLUSIVE;
use crate::locale::config::SAMPLING_SCOPE;

/// 샘플링 항목 이름과 범위. 대상은 모델 이름, 최상위면 기본 표시.
pub(super) fn check_sampling(
    sampling: Option<&Value>,
    model_name: Option<&str>,
) -> Result<(), ValidationError> {
    let Some(sampling) = sampling.filter(|value| !value.is_null()) else {
        return Ok(());
    };
    let scope = model_name.unwrap_or(SAMPLING_SCOPE).to_owned();
    let Some(items) = sampling.as_object() else {
        return Err(ValidationError::SamplingRejected {
            scope,
            item: SAMPLING_SCOPE.to_owned(),
        });
    };
    check_sampling_items(items, &scope)
}

/// 샘플링 항목 하나씩 이름과 범위 확인.
fn check_sampling_items(items: &Map<String, Value>, scope: &str) -> Result<(), ValidationError> {
    for (key, value) in items {
        let is_known = SAMPLING_ITEMS.contains(&key.as_str());
        if !is_known {
            return Err(ValidationError::SamplingRejected {
                scope: scope.to_owned(),
                item: key.to_owned(),
            });
        }
        let is_in_range = match key.as_str() {
            "temperature" => is_number_between(value, TEMPERATURE_MIN, TEMPERATURE_MAX, true),
            "top_p" => is_number_between(value, TOP_P_MIN_EXCLUSIVE, TOP_P_MAX, false),
            _ => is_integer_at_least(value, MAX_TOKENS_MIN),
        };
        if !is_in_range {
            return Err(ValidationError::SamplingRejected {
                scope: scope.to_owned(),
                item: key.to_owned(),
            });
        }
    }
    Ok(())
}
