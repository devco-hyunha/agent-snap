//! 재시도 공개 판단 진입점.
//!
//! [`classify_retry`] → [`interpret_retry_after`] → [`decide_backoff`]/[`resolve_give_up_priority`] 순으로 조립.

use std::time::{Duration, SystemTime};

use super::after::{RetryAfterDecision, interpret_retry_after};
use super::backoff::{BackoffDecision, GiveUpPriority, decide_backoff, resolve_give_up_priority};
use super::classify::classify_retry;
use super::input::RetryFailureInput;

/// 재시도 판단에 쓰는 설정 값.
///
/// `Duration`·`u32`만 담으며 [`crate::config::HttpSettings`]에 의존하지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetrySettings {
    /// 최대 재시도 횟수(첫 요청=1일 때 `retries=3`이면 요청 최대 4).
    pub retries: u32,
    /// Full Jitter 백오프 시작값.
    pub backoff_min: Duration,
    /// Full Jitter 백오프 상한.
    pub backoff_max: Duration,
    /// `Retry-After`를 따르는 상한.
    pub retry_after_max: Duration,
}

/// 공개 판단 결과.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryDecision {
    /// 이만큼 기다린 뒤 재시도.
    Retry {
        /// 대기 시간(0초 포함).
        wait: Duration,
    },
    /// 재시도하지 않음.
    GiveUp(RetryGiveUpReason),
}

/// 포기 이유.
///
/// 키·base_url 호스트·응답 본문은 담지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryGiveUpReason {
    /// 재시도 대상이 아님.
    NotCandidate(NotCandidateDetail),
    /// 최대 시도 횟수를 넘음.
    AttemptsExceeded {
        /// 방금 실패한 요청의 시도 번호(첫 요청=1).
        attempt: u32,
    },
    /// `Retry-After`가 상한을 넘음.
    RetryAfterExceedsCap {
        /// 헤더가 지시한 대기 시간.
        wait: Duration,
    },
}

/// 재시도 대상이 아닐 때의 입력 요약(비밀·본문 없음).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotCandidateDetail {
    /// HTTP 상태 코드.
    HttpStatus {
        /// 상태 코드.
        status: u16,
    },
    /// 응답 헤더 대기 시간 초과.
    HeaderWaitTimeout,
}

/// 실패 입력·시도·설정·난수·시각으로 재시도 여부를 판단.
///
/// # 조립 순서
///
/// 1. `classify_retry` — 비후보면 즉시 포기
/// 2. `interpret_retry_after` — HTTP 후보만(연결 오류는 헤더 없음)
/// 3. `resolve_give_up_priority` / `decide_backoff` — 횟수 우선·백오프
///
/// `retry_after_header`는 원문 헤더. 비후보면 해석하지 않음. `unit_sample`은 Full Jitter용 `[0.0, 1.0]`.
pub fn decide_retry(
    input: RetryFailureInput,
    retry_after_header: Option<&str>,
    attempt: u32,
    settings: RetrySettings,
    now: SystemTime,
    unit_sample: f64,
) -> RetryDecision {
    if !classify_retry(input).is_candidate() {
        // PreHeaderConnect는 후보라 상세가 없음. 방어적으로 후보 경로로 이어감(패닉 없음).
        if let Some(detail) = not_candidate_detail(input) {
            return RetryDecision::GiveUp(RetryGiveUpReason::NotCandidate(detail));
        }
    }

    let after = match input {
        RetryFailureInput::HttpStatus { .. } => {
            interpret_retry_after(retry_after_header, now, settings.retry_after_max)
        }
        RetryFailureInput::PreHeaderConnect(_) | RetryFailureInput::HeaderWaitTimeout => {
            RetryAfterDecision::Absent
        }
    };

    let exceeds_cap = matches!(after, RetryAfterDecision::ExceedsCap { .. });
    if let Some(priority) = resolve_give_up_priority(attempt, settings.retries, exceeds_cap) {
        return give_up_from_priority(priority, after);
    }

    match after {
        RetryAfterDecision::Wait(wait) => RetryDecision::Retry { wait },
        RetryAfterDecision::Absent => match decide_backoff(
            attempt,
            settings.retries,
            settings.backoff_min,
            settings.backoff_max,
            unit_sample,
        ) {
            BackoffDecision::Wait(wait) => RetryDecision::Retry { wait },
            // resolve_give_up_priority가 횟수를 이미 통과시킴.
            BackoffDecision::AttemptsExceeded { attempt } => {
                RetryDecision::GiveUp(RetryGiveUpReason::AttemptsExceeded { attempt })
            }
        },
        // exceeds_cap이면 위에서 이미 반환.
        RetryAfterDecision::ExceedsCap { wait } => {
            RetryDecision::GiveUp(RetryGiveUpReason::RetryAfterExceedsCap { wait })
        }
    }
}

fn give_up_from_priority(priority: GiveUpPriority, after: RetryAfterDecision) -> RetryDecision {
    match priority {
        GiveUpPriority::AttemptsExceeded { attempt } => {
            RetryDecision::GiveUp(RetryGiveUpReason::AttemptsExceeded { attempt })
        }
        GiveUpPriority::RetryAfterExceedsCap => {
            let wait = match after {
                RetryAfterDecision::ExceedsCap { wait } => wait,
                // 우선순위만 ExceedsCap인데 after가 아니면 0초로 고지(방어).
                _ => Duration::ZERO,
            };
            RetryDecision::GiveUp(RetryGiveUpReason::RetryAfterExceedsCap { wait })
        }
    }
}

/// 비후보 입력만 상세로 바꿈. 후보(`PreHeaderConnect`)는 [`None`].
fn not_candidate_detail(input: RetryFailureInput) -> Option<NotCandidateDetail> {
    match input {
        RetryFailureInput::HttpStatus { status, .. } => {
            Some(NotCandidateDetail::HttpStatus { status })
        }
        RetryFailureInput::HeaderWaitTimeout => Some(NotCandidateDetail::HeaderWaitTimeout),
        // 가짜 HeaderWaitTimeout 매핑 금지.
        RetryFailureInput::PreHeaderConnect(_) => None,
    }
}
