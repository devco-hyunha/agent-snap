//! 숫자 값 헬퍼 자리. 범위·정수·시간 값 확인만 봄.

use serde_json::Value;

use crate::consts::limits::SECONDS_MIN_EXCLUSIVE;

/// 시간 값: 숫자이고 0 초과. 상한은 무한대.
pub(super) fn is_seconds(value: &Value) -> bool {
    is_number_between(value, SECONDS_MIN_EXCLUSIVE, f64::INFINITY, false)
}

/// 숫자 값 최소·최대 범위 확인. 최소 포함 여부는 `min_included`로 구분.
pub(super) fn is_number_between(value: &Value, min: f64, max: f64, min_included: bool) -> bool {
    let Some(number) = value.as_f64() else {
        return false;
    };
    let passes_min = if min_included {
        number >= min
    } else {
        number > min
    };
    passes_min && number <= max && number.is_finite()
}

/// 정수 값 최소 이상 확인. 숫자가 아니면 거부.
pub(super) fn is_integer_at_least(value: &Value, min: u64) -> bool {
    let Some(number) = value.as_u64() else {
        return false;
    };
    number >= min
}
