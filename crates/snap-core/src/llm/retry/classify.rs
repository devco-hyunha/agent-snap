//! 재시도 대상 판별입니다.
//!
//! 상태 코드·헤더 전 연결 오류·헤더 대기 초과만으로 후보/비후보를 가립니다.
//! 판단 입력 타입 선언은 [`super::input`]에 두고, 판별 규칙만 이 파일에 둡니다(03-6 R2 감사 분리).
//! 공개 조립은 [`super::decide::decide_retry`]가 맡습니다(D13, D14, D26).

use super::input::RetryFailureInput;

/// 재시도 대상 판별 결과입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetryClass {
    /// 재시도 후보입니다. 횟수·`Retry-After` 상한은 후속이 봅니다.
    Candidate,
    /// 재시도 대상이 아니며 즉시 포기합니다.
    NotCandidate,
}

impl RetryClass {
    /// 재시도 후보이면 `true`입니다.
    pub(crate) const fn is_candidate(self) -> bool {
        matches!(self, Self::Candidate)
    }
}

/// 화이트리스트 HTTP 상태(재시도 후보)입니다.
const RETRY_STATUS_WHITELIST: &[u16] = &[429, 500, 502, 503, 504];

/// 실패 입력만으로 재시도 후보인지 판별합니다.
///
/// 상태 코드는 화이트리스트(429·500/502/503/504)만 후보입니다.
/// 402·408·425·목록 밖 5xx·그 밖의 4xx는 비후보이며, 402는 `Retry-After`가 있어도 같습니다.
/// 헤더 전 연결 오류는 후보, 헤더 대기 초과는 비후보입니다.
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
