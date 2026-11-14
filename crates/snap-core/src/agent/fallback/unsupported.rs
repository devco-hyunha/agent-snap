//! 미지원 상태를 루프 판단 값으로 조립하는 자리입니다(03-6 R1 감사 분리).
//!
//! 상태 코드 표와 형식 변환은 `tool_call_form.rs`가, 폴백 지점 표기는 `connector.rs`가 맡고,
//! 이 파일은 "4x 미지원 코드를 받았는지 확인해 판단 값으로 묶는 조립"만 맡습니다
//! (dev-principles 2절 SRP, 03-6 R1 감사 분리).

use crate::agent::schema::types::LoopError;
use crate::llm::TransportError;

use super::connector::FallbackPoint;
use super::tool_call_form::is_unsupported_status;

/// 4x 미지원 코드를 받았는지 확인해 판단 값을 뽑습니다. 폴백 판단이 아닌 전송 실패는 None입니다.
#[must_use]
pub fn unsupported_point(failure: &LoopError) -> Option<FallbackPoint> {
    let LoopError::Unsupported { point, .. } = failure else {
        return None;
    };
    Some(*point)
}

/// 전송 실패를 루프 오류로 바꿉니다. 4x 미지원 코드는 폴백 연결 지점 판단으로 채우고(AC 4),
/// 그 밖의 전송 실패는 코드를 보존해 그대로 넘깁니다(재전송은 02-6 한 층만 함, D13).
#[must_use]
pub fn transport_failure(error: &TransportError) -> LoopError {
    let transport = LoopError::transport(error);
    // 상태 코드 응답에서만 4x 표를 봅니다. 상태 코드가 없는 실패는 문구만 보존해 그대로 넘깁니다.
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
