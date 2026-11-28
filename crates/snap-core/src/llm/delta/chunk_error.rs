//! 청크 JSON 해석 실패 타입 자리. payload 원문은 밝히지 않고 종류만 담음.

use thiserror::Error;

use crate::locale::sse::CHUNK_ERROR_DATA;
use crate::locale::sse::CHUNK_ERROR_IO_OR_EOF;
use crate::locale::sse::CHUNK_ERROR_SYNTAX;
use crate::locale::sse::CHUNK_PARSE_FAILED;

/// 청크 JSON 해석 실패 상태. payload 원문은 미포함.
#[derive(Debug, Error)]
#[error("{CHUNK_PARSE_FAILED}: {category}")]
pub struct ChunkError {
    /// JSON 오류 종류만 밝힘. payload 원문은 미포함.
    pub category: ChunkErrorKind,
}

impl ChunkError {
    /// serde_json 오류를 종류만 남기는 상태로 옮김.
    pub(super) fn from_json_error(error: &serde_json::Error) -> Self {
        let category = ChunkErrorKind::from_json_error(error);
        Self { category }
    }
}

impl ChunkErrorKind {
    /// serde_json 오류의 종류만 남김. payload 원문은 미포함.
    pub fn from_json_error(error: &serde_json::Error) -> Self {
        match error.classify() {
            serde_json::error::Category::Syntax => Self::Syntax,
            serde_json::error::Category::Data => Self::Data,
            serde_json::error::Category::Io | serde_json::error::Category::Eof => Self::IoOrEof,
        }
    }

    /// 잘린 UTF-8 문자열을 종류만 남김. 잘린 마지막 이벤트의 읽기 실패로 본다.
    pub const fn from_split_error(_error: &crate::llm::split_error::SplitError) -> Self {
        Self::IoOrEof
    }
}

/// 청크 JSON 해석 실패 종류.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkErrorKind {
    /// JSON 문법이 틀린 자리.
    Syntax,

    /// 문법은 맞으나 값 타입이 맞지 않는 자리.
    Data,

    /// 읽기 실패 또는 예상 못한 끝.
    IoOrEof,
}

impl std::fmt::Display for ChunkErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Self::Syntax => CHUNK_ERROR_SYNTAX,
            Self::Data => CHUNK_ERROR_DATA,
            Self::IoOrEof => CHUNK_ERROR_IO_OR_EOF,
        };
        f.write_str(kind)
    }
}
