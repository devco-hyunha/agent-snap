//! 재시도 판별에 넣는 입력 타입 선언.
//!
//! 판별 규칙은 [`super::classify`], 타입 선언은 이 파일.

/// 헤더를 받기 전 연결 단계에서 난 오류 종류.
///
/// TLS·프록시도 헤더 전 연결 후보. 모두 이 enum으로 냄.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreHeaderConnectKind {
    /// DNS 조회 실패.
    DnsFailure,
    /// 연결 거부.
    ConnectionRefused,
    /// 연결 수립 시간 초과.
    ConnectTimeout,
    /// 응답 헤더 전에 연결 재설정.
    ResetBeforeHeaders,
    /// TLS 핸드셰이크 등 헤더 전 TLS 실패.
    Tls,
    /// 프록시 등 헤더 전 연결 단계 실패.
    Proxy,
}

/// 재시도 판별에 넣는 실패 입력. 종류만 정의.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryFailureInput {
    /// HTTP 응답 상태.
    ///
    /// `has_retry_after`는 헤더 존재 여부만. 초·HTTP 날짜는 해석하지 않음. 402는 있어도 비후보.
    HttpStatus {
        /// HTTP 상태 코드.
        status: u16,
        /// `Retry-After` 헤더가 있었는지(값 해석 없음).
        has_retry_after: bool,
    },
    /// 헤더 전 연결 오류(재시도 후보).
    PreHeaderConnect(PreHeaderConnectKind),
    /// 응답 헤더 대기 시간 초과(즉시 포기).
    HeaderWaitTimeout,
}
