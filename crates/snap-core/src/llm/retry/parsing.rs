//! 문자열 자리수 파싱 자리. HTTP-date 파서가 쓰는 순수 함수만 담음.

/// 정해진 자리 수의 10진수를 읽음. 자리 수가 맞지 않으면 `None`.
pub fn parse_fixed_digits(value: &str, digits: usize) -> Option<u32> {
    if value.len() != digits || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    value.parse().ok()
}

/// `HH:MM:SS`를 시·분·초로 바꿈. 윤초(60)까지 허용.
pub fn parse_time_of_day(value: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = value.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let hour = parse_fixed_digits(parts[0], 2)?;
    let minute = parse_fixed_digits(parts[1], 2)?;
    let second = parse_fixed_digits(parts[2], 2)?;
    if hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    Some((hour, minute, second))
}
