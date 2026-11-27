//! 02-5 전송 오류를 02-4 판단 입력으로 옮기는 자리. 02-4 spec 매핑 표의 행만 담음.

use crate::llm::FailureInput;
use crate::llm::client::error::TransportError;

/// 02-5 전송 오류를 02-4 판단 입력으로 옮기는 함수.
///
/// 헤더 전 연결 실패·헤더 대기 초과·HTTP 상태만 판단 입력으로 옮김.
/// 스트림 유휴·헤더 이후 조각 읽기 실패·스트림 오류·취소·끝난 스트림은 판단 입력이 아니므로 `None`.
pub const fn map_failure(error: &TransportError) -> Option<FailureInput> {
    match error {
        TransportError::Connect { kind, .. } => Some(FailureInput::PreHeaderConnect(*kind)),
        TransportError::HeaderWait { .. } => Some(FailureInput::HeaderWaitTimeout),
        TransportError::Status { status, .. } => Some(FailureInput::Status(*status)),
        TransportError::Idle { .. }
        | TransportError::StreamRead { .. }
        | TransportError::Stream(_)
        | TransportError::Cancelled
        | TransportError::Closed => None,
    }
}
