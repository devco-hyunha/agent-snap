//! `Retry-After` 헤더 해석과 상한 비교.
//!
//! delay-seconds·HTTP-date를 대기로 바꾸고 상한과 비교. 재시도 후보일 때만 호출.

use std::time::{Duration, SystemTime};

use super::http_date::parse_imf_fixdate;

/// `Retry-After` 해석·상한 비교 결과.
///
/// 포기 이유에는 대기만 담고 키·호스트·본문은 담지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetryAfterDecision {
    /// 상한 이하이므로 이만큼 기다리라고 권고(0초 포함).
    Wait(Duration),
    /// 헤더가 없거나 음수·해석 불가. 호출측이 백오프 경로로 감.
    Absent,
    /// 상한을 넘는 대기이므로 포기. 대기만 담음.
    ExceedsCap {
        /// 헤더가 지시한 대기 시간.
        wait: Duration,
    },
}

/// `Retry-After` 헤더를 해석하고 상한과 비교.
///
/// # 호출 전제
///
/// [`super::classify::classify_retry`]가 재시도 후보일 때만 호출.
///
/// # 경계
///
/// - `None`·빈 문자열·음수·해석 불가 → [`RetryAfterDecision::Absent`]
/// - 0초 또는 과거 HTTP 날짜 → [`RetryAfterDecision::Wait`]`(0)`
/// - 대기 ≤ 상한 → [`RetryAfterDecision::Wait`]
/// - 대기 > 상한 → [`RetryAfterDecision::ExceedsCap`]
pub(crate) fn interpret_retry_after(
    header: Option<&str>,
    now: SystemTime,
    cap: Duration,
) -> RetryAfterDecision {
    let Some(raw) = header else {
        return RetryAfterDecision::Absent;
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return RetryAfterDecision::Absent;
    }

    let Some(wait) = parse_retry_after_wait(trimmed, now) else {
        return RetryAfterDecision::Absent;
    };

    if wait > cap {
        RetryAfterDecision::ExceedsCap { wait }
    } else {
        RetryAfterDecision::Wait(wait)
    }
}

/// 헤더 본문을 대기 [`Duration`]으로 바꿈. 해석 불가면 `None`.
fn parse_retry_after_wait(value: &str, now: SystemTime) -> Option<Duration> {
    if let Some(secs) = parse_delay_seconds(value) {
        return Some(Duration::from_secs(secs));
    }
    let when = parse_imf_fixdate(value)?;
    Some(wait_until(when, now))
}

/// 목표 시각까지 남은 대기. 과거이면 0초.
pub(crate) fn wait_until(when: SystemTime, now: SystemTime) -> Duration {
    when.duration_since(now).unwrap_or(Duration::ZERO)
}

/// RFC 9110 delay-seconds(비음수 정수)를 읽음. 음수·비숫자는 `None`.
fn parse_delay_seconds(value: &str) -> Option<u64> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    value.parse().ok()
}
