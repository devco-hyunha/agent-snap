//! IMF-fixdate(HTTP-date) 해석 자리. `Retry-After` 정책과 분리한 범용 HTTP-date 파서.
//!
//! 달력 계산은 [`crate::llm::retry::calendar`]를 쓰고, 대기는 [`crate::llm::retry::after`]가 정함.

use std::time::SystemTime;

use crate::llm::retry::calendar::civil_utc_to_epoch_seconds;
use crate::llm::retry::calendar::is_imf_day_name;
use crate::llm::retry::calendar::month_from_name;
use crate::llm::retry::calendar::system_time_from_epoch_seconds;
use crate::llm::retry::parsing::parse_fixed_digits;
use crate::llm::retry::parsing::parse_time_of_day;

/// 날짜·시각·시간대 조각 개수.
const DATE_PARTS: usize = 5;

/// `day-name,` 구분자.
const DAY_NAME_SEPARATOR: &str = ", ";

/// IMF-fixdate 시간대. `GMT`만 받음.
const GMT_ZONE: &str = "GMT";

/// IMF-fixdate(`day-name, DD Mon YYYY HH:MM:SS GMT`)를 읽음.
///
/// 요일 이름은 형식만 확인하고 날짜와 짝이 맞는지는 보지 않음. 형식이 아니면 `None`.
pub fn parse_imf_fixdate(value: &str) -> Option<SystemTime> {
    let (day_name, rest) = value.split_once(DAY_NAME_SEPARATOR)?;
    if !is_imf_day_name(day_name) {
        return None;
    }
    let parts: Vec<&str> = rest.split_whitespace().collect();
    if parts.len() != DATE_PARTS {
        return None;
    }
    if parts[4] != GMT_ZONE {
        return None;
    }
    let (hour, minute, second) = parse_time_of_day(parts[3])?;
    let epoch_seconds = civil_utc_to_epoch_seconds(
        parse_year(parts[2])?,
        month_from_name(parts[1])?,
        parse_day(parts[0])?,
        hour,
        minute,
        second,
    )?;
    system_time_from_epoch_seconds(epoch_seconds)
}

/// `DD`를 날로 읽음. 두 자리 숫자가 아니면 `None`.
fn parse_day(value: &str) -> Option<u32> {
    parse_fixed_digits(value, 2)
}

/// `YYYY`를 해로 읽음. 네 자리 숫자가 아니면 `None`.
fn parse_year(value: &str) -> Option<i32> {
    let year = parse_fixed_digits(value, 4)?;
    i32::try_from(year).ok()
}
