//! LLM 클라이언트 공개 계약. 요청·취소로 이벤트 스트림을 염.
//!
//! 상위는 이 트레이트와 타입만 봄. HTTP 라이브러리 타입은 두지 않음.

mod error;
mod error_debug;
mod frames;
mod give_up_note;
mod outcome;
mod request;
pub(crate) mod schema_openai;
mod segment;
mod sse;
mod stream;
mod timing;
mod traits;

pub(crate) use schema_openai::message_json;

/// 스트림 취소를 알리는 토큰. 복제본이 같은 취소 상태를 나눔.
pub use tokio_util::sync::CancellationToken;

pub use error::TransportError;
pub(crate) use frames::{ChatDriver, ChatFrame};
pub use outcome::StreamOutcome;
pub use request::{AssistantToolCall, ChatMessage, ChatRequest, ToolDefinition};
pub use segment::TimeoutSegment;
pub use sse::{SseItem, SseParser};
pub use stream::ChatStream;
pub use timing::StreamTiming;
pub use traits::LlmClient;
