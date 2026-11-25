//! 재시도 대상 판별 자리. 상태 코드·연결 오류·헤더 대기 초과만 보고 후보 여부를 정함.
//! `Retry-After` 해석·백오프·판정 조립은 이 파일 소관이 아님.

use crate::llm::retry::failures::FailureInput;
use crate::llm::retry::failures::RetryCandidate;

/// 재시도 후보 상태 코드. 429와 500/502/503/504만.
const RETRY_STATUS_CODES: [u16; 5] = [429, 500, 502, 503, 504];

/// 즉시 포기 상태 코드. 402는 `Retry-After`가 있어도 후보가 아님.
const NO_RETRY_STATUS_CODES: [u16; 3] = [402, 408, 425];

/// 재시도 후보 판별
pub fn is_candidate(input: FailureInput) -> RetryCandidate {
    match input {
        FailureInput::PreHeaderConnect(_) => RetryCandidate::Yes,
        FailureInput::HeaderWaitTimeout => RetryCandidate::No,
        FailureInput::Status(code) => status_candidate(code),
    }
}

/// 상태 코드 후보 판별
fn status_candidate(code: u16) -> RetryCandidate {
    if NO_RETRY_STATUS_CODES.contains(&code) {
        return RetryCandidate::No;
    }
    // 후보는 화이트리스트 5종뿐이라 목록 밖 5xx와 그 밖의 4xx(429 제외)는 모두 후보가 아님.
    if RETRY_STATUS_CODES.contains(&code) {
        RetryCandidate::Yes
    } else {
        RetryCandidate::No
    }
}
