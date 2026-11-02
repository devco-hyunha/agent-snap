//! 재시도 대상 판별입니다.
//!
//! 상태 코드·헤더 전 연결 오류·헤더 대기 초과만으로 후보/비후보를 가립니다.
//! 공개 조립은 [`super::decide::decide_retry`]가 맡습니다(D13, D14, D26).

/// 헤더를 받기 전 연결 단계에서 난 오류 종류입니다.
///
/// TLS·프록시는 02-5에 별도 행이 없어 같은 「헤더 전 연결 오류」에 둡니다(D26).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreHeaderConnectKind {
    /// DNS 조회 실패입니다.
    DnsFailure,
    /// 연결이 거부되었습니다.
    ConnectionRefused,
    /// 연결 수립 시간 초과입니다.
    ConnectTimeout,
    /// 응답 헤더 전에 연결이 재설정되었습니다.
    ResetBeforeHeaders,
    /// TLS 핸드셰이크 등 헤더 전 TLS 실패입니다.
    Tls,
    /// 프록시 등 헤더 전 연결 단계 실패입니다.
    Proxy,
}

/// 재시도 판별에 넣는 실패 입력입니다.
///
/// 02-5 오류 → 이 입력으로의 매핑은 02-6이 맡습니다. 여기서는 종류만 정의합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryFailureInput {
    /// HTTP 응답 상태입니다.
    ///
    /// `has_retry_after`는 헤더 존재 여부만 표시합니다. 초·HTTP 날짜 해석은 하지 않습니다.
    /// 402는 이 값이 `true`여도 후보가 아닙니다(D26).
    HttpStatus {
        /// HTTP 상태 코드입니다.
        status: u16,
        /// `Retry-After` 헤더가 있었는지입니다(값 해석 없음).
        has_retry_after: bool,
    },
    /// 헤더 전 연결 오류입니다(재시도 후보).
    PreHeaderConnect(PreHeaderConnectKind),
    /// 응답 헤더 대기 시간 초과입니다(즉시 포기, D14·D26).
    HeaderWaitTimeout,
}

/// 재시도 대상 판별 결과입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryClass {
    /// 재시도 후보입니다. 횟수·`Retry-After` 상한은 후속이 봅니다.
    Candidate,
    /// 재시도 대상이 아니며 즉시 포기합니다.
    NotCandidate,
}

impl RetryClass {
    /// 재시도 후보이면 `true`입니다.
    pub const fn is_candidate(self) -> bool {
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
pub fn classify_retry(input: RetryFailureInput) -> RetryClass {
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
