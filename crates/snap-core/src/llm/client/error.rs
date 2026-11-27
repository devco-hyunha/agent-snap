//! 전송 오류 타입 골격 자리. 키·base_url 호스트는 담지 않음.

use thiserror::Error;

use crate::llm::PreHeaderConnectKind;
use crate::llm::RedactedSummary;
use crate::llm::SseError;
use crate::llm::client::connect_kind::display_name;
use crate::locale::transport::CONNECT_FAILED;
use crate::locale::transport::HEADER_WAIT_TIMEOUT;
use crate::locale::transport::REQUEST_CANCELLED;
use crate::locale::transport::SERVER_ERROR_CHUNK;
use crate::locale::transport::STATUS_FAILED;
use crate::locale::transport::STREAM_CLOSED;
use crate::locale::transport::STREAM_IDLE_TIMEOUT;

/// 스트림을 여는 중의 실패. 02-6이 02-4에 넣을 입력으로 옮기는 일은 02-6 몫.
#[derive(Debug, Error, PartialEq)]
pub enum TransportError {
    /// 헤더를 받기 전 연결 실패. 종류와 연결 설정 초 값만 담음.
    #[error("{CONNECT_FAILED}: {}", display_name(*.kind))]
    Connect {
        /// 02-4 매핑 표의 헤더 전 연결 세부 종류.
        kind: PreHeaderConnectKind,

        /// 연결 시도 설정 초 값.
        connect_seconds: f64,
    },

    /// 서버가 잘못된 HTTP 상태로 응답함. 코드와 가린 요약만 담음.
    #[error("{STATUS_FAILED}: {status}")]
    Status {
        /// HTTP 상태 코드.
        status: u16,

        /// 가린 뒤 길이를 제한한 본문 요약.
        summary: RedactedSummary,
    },

    /// 응답 헤더를 기다리다 시간 초과함.
    #[error("{HEADER_WAIT_TIMEOUT}: {seconds}s")]
    HeaderWait {
        /// 헤더 대기 설정 초 값.
        seconds: f64,
    },

    /// 응답 스트림이 멈춰서 시간 초과함.
    #[error("{STREAM_IDLE_TIMEOUT}: {seconds}s")]
    Idle {
        /// 스트림 유휴 설정 초 값.
        seconds: f64,
    },

    /// 서버가 보낸 오류 청크·형식 오류. 02-3의 `SseError`를 그대로 씀.
    #[error("{SERVER_ERROR_CHUNK}")]
    Stream(#[source] SseError),

    /// 스트림 도중 요청을 취소함.
    #[error("{REQUEST_CANCELLED}")]
    Cancelled,

    /// 이미 끝난 스트림에 대한 조각 거부.
    #[error("{STREAM_CLOSED}")]
    Closed,
}

impl TransportError {
    /// 헤더 전 연결 실패 만들기. 종류와 연결 설정 초 값만 씀.
    pub const fn connect(kind: PreHeaderConnectKind, connect_seconds: f64) -> Self {
        Self::Connect {
            kind,
            connect_seconds,
        }
    }

    /// 잘못된 HTTP 상태 오류 만들기. 코드와 가린 요약만 씀.
    pub const fn status(status: u16, summary: RedactedSummary) -> Self {
        Self::Status { status, summary }
    }

    /// 헤더 대기 시간 초과 만들기. 설정 초 값만 씀.
    pub const fn header_wait(seconds: f64) -> Self {
        Self::HeaderWait { seconds }
    }

    /// 스트림 유휴 시간 초과 만들기. 설정 초 값만 씀.
    pub const fn idle(seconds: f64) -> Self {
        Self::Idle { seconds }
    }

    /// 서버 오류 청크·형식 오류 만들기.
    pub const fn stream(error: SseError) -> Self {
        Self::Stream(error)
    }

    /// 취소 오류 만들기.
    pub const fn cancelled() -> Self {
        Self::Cancelled
    }

    /// 끝난 스트림 오류 만들기.
    pub const fn closed() -> Self {
        Self::Closed
    }
}
