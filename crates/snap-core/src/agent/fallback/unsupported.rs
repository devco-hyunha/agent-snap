//! 미지원 상태를 루프 판단 값으로 조립하는 자리

use crate::agent::schema::types::LoopError;
use crate::llm::TransportError;

use super::connector::FallbackPoint;
use super::tool_call_form::is_unsupported_status;

/// 4x 미지원 코드를 받았는지 확인해 판단 값을 뽑음. 폴백 판단이 아닌 전송 실패는 None
#[must_use]
pub fn unsupported_point(failure: &LoopError) -> Option<FallbackPoint> {
    let LoopError::Unsupported { point, .. } = failure else {
        return None;
    };
    Some(*point)
}

/// 전송 실패를 루프 오류로 변환. 4x 미지원 코드는 폴백 연결 지점 판단으로 채움
#[must_use]
pub fn transport_failure(error: &TransportError) -> LoopError {
    let transport = LoopError::transport(error);
    // 상태 코드 응답에서만 4x 표를 확인. 상태 코드가 없는 실패는 문구만 보존해 그대로 넘김
    let LoopError::Transport {
        reason,
        status: Some(status),
    } = transport
    else {
        return transport;
    };
    if !is_unsupported_status(status) {
        return LoopError::Transport {
            reason,
            status: Some(status),
        };
    }
    LoopError::unsupported(FallbackPoint::UnsupportedStatus { status })
}
