//! `Retry-After` 헤더 해석과 상한 비교입니다.
//!
//! delay-seconds와 HTTP-date(IMF-fixdate)를 대기 시간으로 바꾸고, 설정 상한과 비교합니다.
//! 호출 전제: [`super::classify::classify_retry`] 결과가 재시도 후보일 때만 이 함수를 호출합니다.
//! 비후보 상태의 헤더는 이 모듈에서 보지 않습니다(D14, D26, RFC 9110).

use std::time::{Duration, SystemTime};

use crate::llm::http_date::{parse_imf_fixdate, wait_until};

/// `Retry-After` 해석·상한 비교 결과입니다.
///
/// 포기 이유에는 대기 시간만 담으며 키·호스트·응답 본문은 담지 않습니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryAfterDecision {
    /// 상한 이하이므로 이만큼 기다리라고 권고합니다(0초 포함).
    Wait(Duration),
    /// 헤더가 없거나 음수·해석 불가입니다. 호출측이 백오프 경로로 갑니다.
    Absent,
    /// 상한을 넘는 대기이므로 포기합니다. 대기 시간만 담습니다.
    ExceedsCap {
        /// 헤더가 지시한 대기 시간입니다.
        wait: Duration,
    },
}

/// `Retry-After` 헤더를 해석하고 상한과 비교합니다.
///
/// # 호출 전제
///
/// [`super::classify::classify_retry`]가 재시도 후보일 때만 호출합니다.
///
/// # 경계
///
/// - `None`·빈 문자열·음수·해석 불가 → [`RetryAfterDecision::Absent`]
/// - 0초 또는 과거 HTTP 날짜 → [`RetryAfterDecision::Wait`]`(0)`
/// - 대기 ≤ 상한 → [`RetryAfterDecision::Wait`]
/// - 대기 > 상한 → [`RetryAfterDecision::ExceedsCap`]
pub fn interpret_retry_after(
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

/// 헤더 본문을 대기 [`Duration`]으로 바꿉니다. 해석 불가면 `None`입니다.
fn parse_retry_after_wait(value: &str, now: SystemTime) -> Option<Duration> {
    if let Some(secs) = parse_delay_seconds(value) {
        return Some(Duration::from_secs(secs));
    }
    let when = parse_imf_fixdate(value)?;
    Some(wait_until(when, now))
}

/// RFC 9110 delay-seconds(비음수 정수)를 읽습니다. 음수·비숫자는 `None`입니다.
fn parse_delay_seconds(value: &str) -> Option<u64> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    value.parse().ok()
}
