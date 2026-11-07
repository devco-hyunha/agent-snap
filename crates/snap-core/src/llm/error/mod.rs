//! SSE·델타 해석 오류: 스트림 오류와 형식 오류를 나누고, 키·주소·청크 원문은 담지 않습니다.

pub(in crate::llm) mod models_format;
pub(in crate::llm) mod redact;
pub(in crate::llm) mod scrub;

use std::fmt;

/// 형식 오류의 종류입니다. 청크 원문은 담지 않습니다(D14).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatErrorKind {
    /// 잘못된 UTF-8 바이트입니다.
    InvalidUtf8,
    /// JSON이 아니거나 구조가 맞지 않습니다.
    InvalidJson,
    /// 스트림이 끊겨 마지막 이벤트가 잘렸습니다(D12).
    TruncatedEvent,
}

impl FormatErrorKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::InvalidUtf8 => "잘못된 UTF-8",
            Self::InvalidJson => "잘못된 JSON",
            Self::TruncatedEvent => "잘린 마지막 이벤트",
        }
    }
}

impl fmt::Display for FormatErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// SSE 줄 처리·델타 복원 실패입니다.
///
/// Display·Debug에 API 키, base_url 호스트, 청크 원문을 담지 않습니다(명세 AC 4, D14).
#[derive(thiserror::Error)]
pub enum StreamParseError {
    /// 최상위 `error` 청크로 스트림이 실패했습니다. 요약만 담습니다.
    #[error("스트림 오류: {summary}")]
    Stream {
        /// 키·주소를 가린 뒤 길이를 제한한 요약입니다.
        summary: String,
    },
    /// JSON·UTF-8 형식 오류입니다. 이벤트 번호와 종류만 담습니다.
    #[error("형식 오류: {kind} (이벤트 {event_index}번)")]
    Format {
        /// 1부터 세는 data 이벤트 번호입니다.
        event_index: usize,
        /// 형식 오류 종류입니다.
        kind: FormatErrorKind,
    },
}

impl fmt::Debug for StreamParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stream { summary } => f.debug_struct("Stream").field("summary", summary).finish(),
            Self::Format { event_index, kind } => f
                .debug_struct("Format")
                .field("event_index", event_index)
                .field("kind", kind)
                .finish(),
        }
    }
}
