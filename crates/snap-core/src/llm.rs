//! OpenAI 호환 엔드포인트와 통신하는 스트리밍 클라이언트 모듈입니다.
//!
//! SSE 줄 처리·델타 복원(02-3), 재시도 판단·반복·`codec::http_date`(02-4·작업 03),
//! 클라이언트 트레이트·전송(02-5)을 둡니다. 재시도 sleep은 02-6입니다.

mod client;
pub(in crate::llm) mod codec;
pub(in crate::llm) mod error;
mod request_body;
pub(crate) mod retry;
pub mod selection;
mod transport;

pub use client::{
    CancellationToken, ChatMessage, ChatRequest, ChatStream, LlmClient, SseItem, SseParser,
    StreamOutcome, StreamTiming, TimeoutSegment, ToolDefinition, TransportError,
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
