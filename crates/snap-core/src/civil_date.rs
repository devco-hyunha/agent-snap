//! UTC 민간 달력 지식(요일·월 이름·윤년·일수)을 한 곳에 담은 자리입니다.
//!
//! LLM이나 HTTP에 묶이지 않은 달력 계산만 두고, 쓰는 쪽(`llm::retry::http_date`)에서 부릅니다.
//! 달력을 고칠 이유가 HTTP 파싱과 다르므로 `llm` 아래가 아닌 crate 최상위에 둡니다.

/// IMF-fixdate 요일 이름(`Mon`~`Sun`)인지 확인합니다.
pub(crate) fn is_imf_day_name(name: &str) -> bool {
    matches!(name, "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun")
}

/// 월 약어(`Jan`~`Dec`)를 1~12 달 번호로 바꿉니다.
pub(crate) fn month_from_name(name: &str) -> Option<u32> {
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

/// `HH:MM:SS`를 시·분·초로 바꿉니다. 초는 윤초(60)까지 허용합니다.
pub(crate) fn parse_time_of_day(value: &str) -> Option<(u32, u32, u32)> {
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
pub(crate) fn civil_utc_to_unix_secs(
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

/// 주한 달의 날 수입니다. 달 번호가 1~12가 아니면 `None`입니다.
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
