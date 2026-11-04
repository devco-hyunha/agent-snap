//! LLM 클라이언트 공개 계약: 요청·취소로 이벤트 스트림을 엽니다.
//!
//! 상위(에이전트 루프 등)는 이 트레이트와 타입만 봅니다.
//! HTTP 라이브러리 타입은 여기 두지 않습니다.

mod error;
mod frames;
mod outcome;
mod request;
mod segment;
mod stream;
mod timing;
mod traits;

/// 스트림 취소를 알리는 토큰입니다. 복제본이 같은 취소 상태를 나눕니다.
pub use tokio_util::sync::CancellationToken;

pub use error::TransportError;
pub(crate) use frames::{ChatDriver, ChatFrame};
pub use outcome::StreamOutcome;
pub use request::{ChatMessage, ChatRequest, ToolDefinition};
pub use segment::TimeoutSegment;
pub use stream::ChatStream;
pub use timing::StreamTiming;
pub use traits::LlmClient;
