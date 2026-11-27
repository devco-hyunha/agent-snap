//! 02-6 전송 오류 타입 시험 재료 자리. 가린 요약·헤더 값·연결 종류·오류 목록 조립만 담음.
//! 같은 시험 묶음의 담는 값 시험과 가리기 시험이 재사용하는 재료 파일.

use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::RedactedSummary;
use snap_core::llm::RetryAfterHeader;
use snap_core::llm::SseError;
use snap_core::llm::TransportError;

/// 시험용 가린 요약.
pub(crate) fn summary() -> RedactedSummary {
    RedactedSummary::new("서버이 오류입니다".to_owned())
}

/// 시험용 `Retry-After` 헤더 값.
pub(crate) fn header(value: &str) -> RetryAfterHeader {
    RetryAfterHeader::new(value.to_owned())
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

/// 시험용 전송 오류 목록.
pub(crate) fn all_errors() -> Vec<TransportError> {
    vec![
        TransportError::connect(PreHeaderConnectKind::Dns, 10.0),
        TransportError::status(503, summary(), None),
        TransportError::header_wait(300.0),
        TransportError::idle(120.0),
        TransportError::stream_read(10.0),
        TransportError::stream(SseError::stream(summary())),
        TransportError::cancelled(),
    ]
}
