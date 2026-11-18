//! 전송 오류와 시간 설정을 재시도 판단 입력으로 바꾸는 매핑.
//!
//! 매핑만 옮기고 재시도 규칙은 정의하지 않음. `Retry-After` 유무는 매개변수로 받음.

use crate::config::HttpSettings;
use crate::llm::client::{TimeoutSegment, TransportError};
use crate::llm::retry::{RetryFailureInput, RetrySettings};

/// 전송 오류를 재시도 판단 입력으로 바꿈.
///
/// 연결 실패·서버 상태·헤더 대기 초과만 입력. 취소·본문 읽기·스트림 유휴·제작 실패·목록 형식은 `None`.
pub(crate) fn retry_input(
    error: &TransportError,
    has_retry_after: bool,
) -> Option<RetryFailureInput> {
    match error {
        TransportError::ConnectFailed { kind, .. } => {
            Some(RetryFailureInput::PreHeaderConnect(*kind))
        }
        TransportError::Timeout { segment, .. } => match *segment {
            TimeoutSegment::HeaderWait => Some(RetryFailureInput::HeaderWaitTimeout),
            TimeoutSegment::StreamIdle => None,
        },
        TransportError::ServerStatus { status, .. } => Some(RetryFailureInput::HttpStatus {
            status: *status,
            has_retry_after,
        }),
        // 재시도 판단 전에 이미 끝난 실패는 입력이 없음.
        TransportError::ClientBuild
        | TransportError::Cancelled
        | TransportError::Streaming(_)
        | TransportError::BodyRead
        // 목록 조회는 재시도 고리를 쓰지 않음.
        | TransportError::ModelsListFormat(_) => None,
    }
}

/// `http` 설정을 재시도 판단이 쓰는 값으로 옮김.
///
/// 필드 4개를 값만 1대1로 옮김. 판단이 설정 타입에 의존하지 않게 막음.
pub(crate) fn retry_settings(http: &HttpSettings) -> RetrySettings {
    RetrySettings {
        retries: http.retries,
        backoff_min: http.backoff_min,
        backoff_max: http.backoff_max,
        retry_after_max: http.retry_after_max,
    }
}
