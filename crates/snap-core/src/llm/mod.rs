//! LLM 연결 자리. SSE 줄 처리 파일을 한자리로 올리고, 델타 복원만 하위 폴더로 둠.

mod delta;
mod error;
mod event;
mod lines;
mod redact;
mod redacted;
mod split_error;
mod stream;
mod summary;

pub use delta::ChunkDelta;
pub use delta::ChunkError;
pub use delta::ChunkErrorKind;
pub use delta::Delta;
pub use delta::DeltaState;
pub use delta::FieldText;
pub use delta::ToolCall;
pub use delta::ToolCallDelta;
pub use delta::Usage;
pub use delta::parse_chunk;
pub use error::SseError;
pub use event::DONE_MARKER;
pub use event::EventKind;
pub use event::SseEvent;
pub use lines::LineState;
pub use lines::Lines;
pub use redacted::RedactedSummary;
pub use split_error::SplitError;
pub use stream::StreamState;
pub use summary::SUMMARY_MAX_CHARS;
pub use summary::redact_summary;
