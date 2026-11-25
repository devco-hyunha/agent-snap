//! 스트림 오류·형식 오류 타입 자리. 키·주소·청크 원문은 미포함.

use thiserror::Error;

use crate::llm::ChunkErrorKind;
use crate::llm::RedactedSummary;
use crate::locale::sse::FORMAT_ERROR_FAILED;
use crate::locale::sse::STREAM_ERROR_FAILED;

/// SSE 스트림 전체 실패 상태.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SseError {
    /// 최상위 `error` 청크가 만든 스트림 오류. 서버가 준 요약만 담음(가린 값).
    #[error("{STREAM_ERROR_FAILED}: {summary}")]
    Stream {
        /// 가린 서버 오류 요약. 키·주소는 가렸음 표시로 바꿈.
        summary: RedactedSummary,
    },

    /// 잘못된 JSON 청크·잘린 마지막 이벤트가 만든 형식 오류. 위치와 종류만 담음.
    #[error("{FORMAT_ERROR_FAILED}: event {index} {kind}")]
    Format {
        /// 오류가 생긴 이벤트 번호(1부터 시작).
        index: usize,

        /// JSON 오류 종류. payload 원문은 미포함.
        kind: ChunkErrorKind,
    },
}

impl SseError {
    /// 형식 오류 만들기. 이벤트 번호와 JSON 오류 종류만 씀.
    pub const fn format(index: usize, kind: ChunkErrorKind) -> Self {
        Self::Format { index, kind }
    }

    /// 스트림 오류 만들기. 가린 서버 오류 요약만 씀.
    pub const fn stream(summary: RedactedSummary) -> Self {
        Self::Stream { summary }
    }
}
