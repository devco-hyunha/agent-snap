//! OpenAI 호환 엔드포인트와 통신하는 스트리밍 클라이언트 모듈.

mod client;
pub(in crate::llm) mod codec;
pub(in crate::llm) mod error;
mod reasoning_wire;
mod request_body;
pub(crate) mod retry;
pub mod selection;
mod transport;

pub use client::{
    AssistantToolCall, CancellationToken, ChatMessage, ChatRequest, ChatStream, LlmClient, SseItem,
    SseParser, StreamOutcome, StreamTiming, TimeoutSegment, ToolDefinition, TransportError,
};
pub use codec::decode::StreamDecoder;
pub use codec::delta::{AssembledResponse, DeltaAssembler, StreamEvent, ToolCall, Usage};
pub use error::{FormatErrorKind, StreamParseError};
pub use retry::{
    NotCandidateDetail, PreHeaderConnectKind, RetryDecision, RetryFailureInput, RetryGiveUpReason,
    RetrySettings, decide_retry,
};
pub use selection::{SelectError, SelectedClient, SelectedModel};
pub use transport::OpenAiCompatClient;

#[cfg(test)]
mod tests;
