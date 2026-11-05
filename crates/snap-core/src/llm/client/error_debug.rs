//! 전송 오류 타입 `TransportError`의 Debug 표기 책임입니다.
//!
//! 키·호스트가 섞인 본문 요약은 이미 가려진 값만 그대로 보여 줍니다.
//! Display 문구와 따로 두는 이유: 표기 형식이 서로 다른 이유로 바뀔 수 있어서입니다(작업 07 SRP 분리).

use std::fmt;

use super::error::TransportError;

impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClientBuild => f.write_str("ClientBuild"),
            Self::Cancelled => f.write_str("Cancelled"),
            Self::ConnectFailed {
                kind,
                connect_seconds,
            } => f
                .debug_struct("ConnectFailed")
                .field("kind", kind)
                .field("connect_seconds", connect_seconds)
                .finish(),
            Self::Timeout { segment, seconds } => f
                .debug_struct("Timeout")
                .field("segment", segment)
                .field("seconds", seconds)
                .finish(),
            Self::ServerStatus {
                status,
                body_summary,
                give_up,
            } => f
                .debug_struct("ServerStatus")
                .field("status", status)
                .field("body_summary", body_summary)
                .field("give_up", give_up)
                .finish(),
            Self::Streaming(error) => f
                .debug_struct("Streaming")
                .field("error", &format!("{error}"))
                .finish(),
            Self::BodyRead => f.write_str("BodyRead"),
        }
    }
}
