//! 재시도 입력 타입 자리. 02-5 전송 오류가 그대로 매핑할 이름만 둠.

/// 헤더를 받기 전 연결 단계에서 생긴 오류 종류.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreHeaderConnectKind {
    /// DNS 이름을 해석하지 못함.
    Dns,

    /// 서버가 연결을 거부함.
    Refused,

    /// 연결 시도 시간 초과.
    ConnectTimeout,

    /// 헤더를 받기 전에 연결이 끊김.
    Reset,

    /// TLS 연결 실패.
    Tls,

    /// 프록시 경유 실패.
    Proxy,
}

/// 재시도 판단이 받는 실패 종류.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureInput {
    /// 서버가 돌려준 HTTP 상태 코드.
    Status(u16),

    /// 헤더를 받기 전 연결 단계에서 생긴 오류.
    PreHeaderConnect(PreHeaderConnectKind),

    /// 응답 헤더를 기다리다 시간 초과함.
    HeaderWaitTimeout,
}

/// 재시도 대상 판별 결과.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryCandidate {
    /// 재시도 후보.
    Yes,

    /// 재시도 대상 아님.
    No,
}
