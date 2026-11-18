//! IMF-fixdate(HTTP-date) 파싱.
//!
//! Retry-After 정책과 분리된 범용 HTTP-date. 달력은 [`crate::civil_date`], 대기는 [`super::after`].

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::civil_date::{
    civil_utc_to_unix_secs, is_imf_day_name, month_from_name, parse_time_of_day,
};

/// IMF-fixdate(`day-name, DD Mon YYYY HH:MM:SS GMT`)만 읽음.
///
/// 요일 이름은 형식만 확인하고 날짜와 일치하는지는 보지 않음.
pub(crate) fn parse_imf_fixdate(value: &str) -> Option<SystemTime> {
    let (day_name, rest) = value.split_once(", ")?;
    if !is_imf_day_name(day_name) {
        return None;
    }

    let parts: Vec<&str> = rest.split_whitespace().collect();
    if parts.len() != 5 {
        return None;
    }
    let (day_s, month_s, year_s, time_s, zone) = (parts[0], parts[1], parts[2], parts[3], parts[4]);
    if zone != "GMT" {
        return None;
    }
    if day_s.len() != 2 || !day_s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let day: u32 = day_s.parse().ok()?;
    let month = month_from_name(month_s)?;
    if year_s.len() != 4 || !year_s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year: i32 = year_s.parse().ok()?;
    let (hour, minute, second) = parse_time_of_day(time_s)?;
    let unix_secs = civil_utc_to_unix_secs(year, month, day, hour, minute, second)?;
    system_time_from_unix_secs(unix_secs)
}

/// Unix 초를 [`SystemTime`]으로 바꿈. 음수(epoch 이전)도 과거 시각으로 담음.
fn system_time_from_unix_secs(unix_secs: i64) -> Option<SystemTime> {
    if unix_secs >= 0 {
        return Some(UNIX_EPOCH + Duration::from_secs(unix_secs as u64));
    }
    // 플랫폼이 epoch 이전을 못 담으면 UNIX_EPOCH로 두어 Wait(0)이 되게 함.
    UNIX_EPOCH
        .checked_sub(Duration::from_secs(unix_secs.unsigned_abs()))
        .or(Some(UNIX_EPOCH))
}
