//! 02-4 판단 입력 매핑 시험 재료 자리. 가린 요약·연결 종류 6종만 담음.
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::RedactedSummary;

/// 시험용 가린 요약.
pub(crate) fn summary() -> RedactedSummary {
    RedactedSummary::new("서버이 오류입니다".to_owned())
}

/// 헤더 전 연결 종류 6종.
pub(crate) fn connect_kinds() -> [PreHeaderConnectKind; 6] {
    [
        PreHeaderConnectKind::Dns,
        PreHeaderConnectKind::Refused,
        PreHeaderConnectKind::ConnectTimeout,
        PreHeaderConnectKind::Reset,
        PreHeaderConnectKind::Tls,
        PreHeaderConnectKind::Proxy,
    ]
}
