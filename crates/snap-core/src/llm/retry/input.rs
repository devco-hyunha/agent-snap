//! 재시도 판별에 넣는 입력 타입 선언만 둡니다(03-6 R2 감사 분리).
//!
//! 판단 규칙([`super::classify`])은 판별 규칙 파일에 두고, 타입 선언은 이 파일에 둡니다
//! (dev-principles 5절 관심사 분리).

/// 헤더를 받기 전 연결 단계에서 난 오류 종류입니다.
///
/// TLS·프록시도 헤더 전 연결 후보입니다. 02-5 오류 표에 행이 있고 모두 이 enum으로 냅니다(D26).
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
