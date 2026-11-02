//! IMF-fixdate(HTTP-date) 파싱과 민간 UTC 달력 헬퍼입니다.
//!
//! Retry-After 정책과 분리된 범용 HTTP-date 관심사입니다(D27).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 목표 시각까지 남은 대기입니다. 과거이면 0초입니다.
pub(crate) fn wait_until(when: SystemTime, now: SystemTime) -> Duration {
    when.duration_since(now).unwrap_or(Duration::ZERO)
}

/// IMF-fixdate(`day-name, DD Mon YYYY HH:MM:SS GMT`)만 읽습니다.
///
/// 요일 이름은 형식만 확인하고 날짜와 일치하는지는 보지 않습니다.
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

/// Unix 초를 [`SystemTime`]으로 바꿉니다. 음수(epoch 이전)도 과거 시각으로 담습니다.
fn system_time_from_unix_secs(unix_secs: i64) -> Option<SystemTime> {
    if unix_secs >= 0 {
        return Some(UNIX_EPOCH + Duration::from_secs(unix_secs as u64));
    }
    // 플랫폼이 epoch 이전을 못 담으면 UNIX_EPOCH로 두어 Wait(0)이 되게 합니다(AC2/D14).
    UNIX_EPOCH
        .checked_sub(Duration::from_secs(unix_secs.unsigned_abs()))
        .or(Some(UNIX_EPOCH))
}

fn is_imf_day_name(name: &str) -> bool {
    matches!(name, "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun")
}

fn month_from_name(name: &str) -> Option<u32> {
    match name {
        "Jan" => Some(1),
        "Feb" => Some(2),
        "Mar" => Some(3),
        "Apr" => Some(4),
        "May" => Some(5),
        "Jun" => Some(6),
        "Jul" => Some(7),
        "Aug" => Some(8),
        "Sep" => Some(9),
        "Oct" => Some(10),
        "Nov" => Some(11),
        "Dec" => Some(12),
        _ => None,
    }
}

fn parse_time_of_day(value: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = value.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let parse_two = |s: &str| -> Option<u32> {
        if s.len() != 2 || !s.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        s.parse().ok()
    };
    let hour = parse_two(parts[0])?;
    let minute = parse_two(parts[1])?;
    let second = parse_two(parts[2])?;
    if hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    Some((hour, minute, second))
}

/// UTC 민간 시각을 Unix 초(부호 있음)로 바꿉니다. 잘못된 날짜면 `None`입니다.
///
/// epoch 이전은 음수이며, 호출측이 과거(대기 0초)로 취급합니다(AC2/D14).
fn civil_utc_to_unix_secs(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Option<i64> {
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month)? {
        return None;
    }
    // Howard Hinnant days_from_civil.
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = (y - era * 400) as u32;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = (era * 146_097 + doe as i32) - 719_468;
    let day_secs = i64::from(hour) * 3600 + i64::from(minute) * 60 + i64::from(second);
    Some(i64::from(days) * 86_400 + day_secs)
}

fn days_in_month(year: i32, month: u32) -> Option<u32> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap_year(year) { 29 } else { 28 }),
        _ => None,
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
