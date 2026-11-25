//! `Retry-After` 헤더 해석과 상한 비교 자리.
//!
//! 헤더가 없거나 해석할 수 없으면 헤더 없음으로 봄. 상한을 넘으면 대기 초를 밝힌 포기로 봄.

use std::time::Duration;
use std::time::SystemTime;

use crate::llm::retry::http_date::parse_imf_fixdate;
use crate::llm::retry::outcome::RetryAfterOutcome;

/// `Retry-After` 헤더를 해석하고 상한과 비교함.
///
/// 해석 불가·음수는 헤더 없음, 0초와 과거 HTTP 날짜는 대기 0초로 봄.
pub fn interpret_retry_after(
    header: Option<&str>,
    now: SystemTime,
    cap: Duration,
) -> RetryAfterOutcome {
    let Some(raw) = header.map(str::trim) else {
        return RetryAfterOutcome::Absent;
    };
    let Some(wait) = parse_retry_after_wait(raw, now) else {
        return RetryAfterOutcome::Absent;
    };
    if wait > cap {
        return RetryAfterOutcome::ExceedsCap { wait };
    }
    RetryAfterOutcome::Wait(wait)
}

/// 헤더 본문을 대기 시간으로 바꿈. 해석할 수 없으면 `None`.
fn parse_retry_after_wait(value: &str, now: SystemTime) -> Option<Duration> {
    if let Some(seconds) = parse_delay_seconds(value) {
        return Some(Duration::from_secs(seconds));
    }
    let when = parse_imf_fixdate(value)?;
    Some(wait_until(when, now))
}

/// RFC 9110의 delay-seconds(음이 아닌 10진 정수)를 읽음. 음수·숫자 외는 `None`.
fn parse_delay_seconds(value: &str) -> Option<u64> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    value.parse().ok()
}

/// 목표 시각까지 남은 대기. 과거면 0초.
fn wait_until(when: SystemTime, now: SystemTime) -> Duration {
    when.duration_since(now).unwrap_or(Duration::ZERO)
}
