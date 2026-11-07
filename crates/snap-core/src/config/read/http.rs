//! 시간·재시도 칸(`http`) 읽기입니다(D20, D21).
//!
//! 인정하는 키는 7개뿐이며, 오류에는 항목 이름만 담고 입력 값은 담지 않습니다.

use std::time::Duration;

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::{ConfigError, ValidationError};
use super::types::HttpSettings;
use super::validate;
use crate::jsonc::{WrongType, parse_number, pick_object, take_optional};

/// 루트의 `http`를 읽습니다. 칸이 없으면 기본값이고, 빠진 항목은 기본값이 됩니다.
pub(super) fn read_http(root: &mut JsonObject<'_>) -> Result<HttpSettings, ConfigError> {
    let mut http = HttpSettings::default();
    let Some(object) = take_optional(root, "http", pick_object)
        .map_err(|WrongType| ConfigError::invalid_field("http"))?
    else {
        return Ok(http);
    };
    for (key, value) in object {
        let item = key.as_ref();
        let invalid = || ConfigError::invalid_field(format!("http.{item}"));
        if item == "retries" {
            http.retries = parse_number(&value).ok_or_else(invalid)?;
            continue;
        }
        let slot =
            duration_slot(&mut http, item).ok_or_else(|| ValidationError::UnknownHttpItem {
                item: item.to_owned(),
            })?;
        *slot = parse_seconds(&value).ok_or_else(invalid)?;
    }
    if !validate::is_valid_backoff_range(http.backoff_min, http.backoff_max) {
        return Err(ValidationError::BackoffRangeInverted.into());
    }
    Ok(http)
}

/// 시간 항목 이름으로 값을 담을 칸을 고릅니다. 시간 항목이 아니면 `None`입니다.
fn duration_slot<'a>(http: &'a mut HttpSettings, item: &str) -> Option<&'a mut Duration> {
    match item {
        "backoff_min" => Some(&mut http.backoff_min),
        "backoff_max" => Some(&mut http.backoff_max),
        "retry_after_max" => Some(&mut http.retry_after_max),
        "connect_seconds" => Some(&mut http.connect_seconds),
        "header_wait_seconds" => Some(&mut http.header_wait_seconds),
        "stream_idle_seconds" => Some(&mut http.stream_idle_seconds),
        _ => None,
    }
}

/// 초 단위 숫자(소수 가능)를 시간으로 바꿉니다. 0·음수·무한대는 거부합니다.
fn parse_seconds(value: &JsonValue<'_>) -> Option<Duration> {
    Duration::try_from_secs_f64(parse_number(value)?)
        .ok()
        .filter(validate::is_positive_duration)
}
