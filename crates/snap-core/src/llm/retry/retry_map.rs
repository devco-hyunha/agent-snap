//! 전송 오류와 시간 설정을 02-4 재시도 판단 입력으로 바꾸는 매핑입니다.
//!
//! 매핑 표만 옮기고 재시도 규칙은 정의하지 않습니다(02-4 `decide_retry` 재사용).
//! 이 함수들은 `retry_loop`이 판단 입력을 만드는 데만 씁니다.
//! `Retry-After` 헤더 유무는 공개 오류가 아니라 전송 내부에서 아는 값이라 매개변수로 따로 받습니다(02-6 spec 33행).

use crate::config::HttpSettings;
use crate::llm::client::{TimeoutSegment, TransportError};
use crate::llm::retry::{RetryFailureInput, RetrySettings};

/// 전송 오류를 재시도 판단 입력으로 바꿉니다.
///
/// 매핑 표대로 옮깁니다: 연결 실패는 헤더 전 연결 후보, 서버 상태 응답은 상태 코드와 `Retry-After`
/// 헤더 유무, 헤더 대기 초과는 비후보 판단 입력으로 봅니다.
/// 취소·본문 읽기 중 실패·스트림 유휴 초과·클라이언트 제작 실패는 판단 입력이 없어 `None`입니다.
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
        // 재시도 판단 전에 이미 끝난 실패(취소·스트림 중 오류 등)는 입력이 없다.
        TransportError::ClientBuild
        | TransportError::Cancelled
        | TransportError::Streaming(_)
        | TransportError::BodyRead => None,
    }
}

/// `http` 설정을 재시도 판단이 쓰는 값으로 옮깁니다.
///
/// 필드 4개를 값만 1대1로 옮깁니다. 재시도 판단이 설정 타입에 의존하지 않게 막는 매핑책입니다
/// (decide.rs 독스트링의 매핑책).
pub(crate) fn retry_settings(http: &HttpSettings) -> RetrySettings {
    RetrySettings {
        retries: http.retries,
        backoff_min: http.backoff_min,
        backoff_max: http.backoff_max,
        retry_after_max: http.retry_after_max,
    }
}
