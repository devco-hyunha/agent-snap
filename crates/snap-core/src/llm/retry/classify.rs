//! 재시도 대상 판별.
//!
//! 상태 코드·헤더 전 연결 오류·헤더 대기 초과만으로 후보/비후보를 가림. 공개 조립은 [`super::decide::decide_retry`].

use super::input::RetryFailureInput;

/// 재시도 대상 판별 결과.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetryClass {
    /// 재시도 후보. 횟수·`Retry-After` 상한은 후속이 봄.
    Candidate,
    /// 재시도 대상이 아니며 즉시 포기.
    NotCandidate,
}

impl RetryClass {
    /// 재시도 후보이면 `true`.
    pub(crate) const fn is_candidate(self) -> bool {
        matches!(self, Self::Candidate)
    }
}

/// 화이트리스트 HTTP 상태(재시도 후보).
const RETRY_STATUS_WHITELIST: &[u16] = &[429, 500, 502, 503, 504];

/// 실패 입력만으로 재시도 후보인지 판별.
///
/// 화이트리스트(429·500/502/503/504)만 후보. 402는 `Retry-After`가 있어도 비후보.
/// 헤더 전 연결 오류는 후보, 헤더 대기 초과는 비후보.
pub(crate) fn classify_retry(input: RetryFailureInput) -> RetryClass {
    match input {
        RetryFailureInput::HttpStatus { status, .. } => classify_http_status(status),
        RetryFailureInput::PreHeaderConnect(_) => RetryClass::Candidate,
        RetryFailureInput::HeaderWaitTimeout => RetryClass::NotCandidate,
    }
}

fn classify_http_status(status: u16) -> RetryClass {
    if RETRY_STATUS_WHITELIST.contains(&status) {
        RetryClass::Candidate
    } else {
        RetryClass::NotCandidate
    }
}
