//! OpenAI 호환 엔드포인트와 통신하는 스트리밍 클라이언트 모듈입니다.
//!
//! SSE 줄 처리·델타 복원(02-3), 재시도 판단·`http_date`(02-4·작업 03),
//! 클라이언트 트레이트·전송(02-5)을 둡니다. 재시도 sleep은 02-6입니다.

mod chunk;
mod client;
mod decode;
mod delta;
mod error;
mod http_date;
mod request_body;
pub(crate) mod retry;
mod sse;
mod transport;

pub use client::{
    CancellationToken, ChatMessage, ChatRequest, ChatStream, LlmClient, StreamOutcome,
    StreamTiming, TimeoutSegment, ToolDefinition, TransportError,
};
pub use decode::StreamDecoder;
pub use delta::{AssembledResponse, DeltaAssembler, StreamEvent, ToolCall, Usage};
pub use error::{FormatErrorKind, StreamParseError};
pub use retry::{
    NotCandidateDetail, PreHeaderConnectKind, RetryDecision, RetryFailureInput, RetryGiveUpReason,
    RetrySettings, decide_retry,
};
pub use sse::{SseItem, SseParser};
pub use transport::OpenAiCompatClient;

#[cfg(test)]
mod tests {
    mod delta;
    mod errors;
    mod mock_server;
    mod redact;
    mod request;
    mod retry;
    mod sse;
    mod support;
    mod transport;
}
