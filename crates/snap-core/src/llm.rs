//! OpenAI 호환 엔드포인트와 통신하는 스트리밍 클라이언트 모듈입니다.
//!
//! 02-3은 네트워크 없이 SSE 줄 처리와 델타 복원만 제공합니다. HTTP 전송은 02-5에서
//! 이 결과를 그대로 씁니다.

mod chunk;
mod decode;
mod delta;
mod error;
mod http_date;
mod retry;
mod sse;

pub use decode::StreamDecoder;
pub use delta::{AssembledResponse, DeltaAssembler, StreamEvent, ToolCall, Usage};
pub use error::{FormatErrorKind, StreamParseError};
pub use retry::{
    BackoffDecision, GiveUpPriority, NotCandidateDetail, PreHeaderConnectKind, RetryAfterDecision,
    RetryClass, RetryDecision, RetryFailureInput, RetryGiveUpReason, RetrySettings,
    capped_exponential_ceiling, classify_retry, decide_backoff, decide_retry, full_jitter_wait,
    interpret_retry_after, is_attempts_exceeded, resolve_give_up_priority,
};
pub use sse::{SseItem, SseParser};

#[cfg(test)]
mod tests {
    mod delta;
    mod errors;
    mod redact;
    mod retry;
    mod sse;
    mod support;
}
