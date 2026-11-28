//! UTC 달력 지식 자리. 요일·월 이름·윤년·월 날 수와 epoch 초 변환만 담음.
//!
//! 외부 크레이트 달력 대신 std 시간 타입과 손으로 계산한 달력을 씀.

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// IMF-fixdate 요일 이름.
const DAY_NAMES: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// 월 약어.
const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// 1970-01-01까지의 기준 날 수.
const EPOCH_DAY_NUMBER: i64 = 719_468;

/// 윤년 규칙. 4로 나누어지고 100으로 나누어지지 않거나 400으로 나누어지면 윤년.
fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// IMF-fixdate 요일 이름을 확인만 함. 날짜와 짝이 맞는지는 보지 않음.
pub fn is_imf_day_name(name: &str) -> bool {
    DAY_NAMES.contains(&name)
}

/// 월 약어를 1~12 달 번호로 바꿈.
pub fn month_from_name(name: &str) -> Option<u32> {
    let index = MONTH_NAMES.iter().position(|&month| month == name)?;
    u32::try_from(index + 1).ok()
}

/// UTC 민간 시각을 1970-01-01부터의 초로 바꿈. epoch 이전은 음수.
///
/// 달 번호·날·시각이 범위를 벗어나면 `None`.
pub fn civil_utc_to_epoch_seconds(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Option<i64> {
    if month == 0 || month > 12 {
        return None;
    }
    let month_days = days_in_month(year, month)?;
    if day == 0 || day > month_days {
        return None;
    }
    let days = civil_days_from_epoch(year, month, day)?;
    let day_seconds = i64::from(hour) * 3_600 + i64::from(minute) * 60 + i64::from(second);
    days.checked_mul(86_400)?.checked_add(day_seconds)
}

/// 1970-01-01부터의 초를 [`SystemTime`]으로 바꿈.
///
/// epoch 이전 초는 음수로 받고, 플랫폼이 담지 못하면 `None`.
pub fn system_time_from_epoch_seconds(epoch_seconds: i64) -> Option<SystemTime> {
    if let Ok(seconds) = u64::try_from(epoch_seconds) {
        return UNIX_EPOCH.checked_add(Duration::from_secs(seconds));
    }
    let past = u64::try_from(-epoch_seconds).ok()?;
    UNIX_EPOCH.checked_sub(Duration::from_secs(past))
}

/// 주어진 달의 날 수.
fn days_in_month(year: i32, month: u32) -> Option<u32> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap_year(year) { 29 } else { 28 }),
        _ => None,
    }
}

/// 1970-01-01부터의 날 수. 3월을 해의 첫 달로 보는 계산.
fn civil_days_from_epoch(year: i32, month: u32, day: u32) -> Option<i64> {
    let shifted_year = if month < 3 { year - 1 } else { year };
    let era = i64::from(shifted_year.div_euclid(400));
    let year_of_era = u64::try_from(i64::from(shifted_year) - era * 400).ok()?;
    let shifted_month = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = u64::from((153 * shifted_month + 2) / 5 + day - 1);
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(era * 146_097 + day_of_era as i64 - EPOCH_DAY_NUMBER)
}
