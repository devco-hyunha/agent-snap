//! OpenAI 호환 엔드포인트와 통신하는 스트리밍 클라이언트 모듈입니다.
//!
//! SSE 줄 처리·델타 복원(02-3), 재시도 판단·`http_date`(02-4·작업 03)을 둡니다.
//! HTTP 전송·sleep은 02-5/02-6에서 이 결과를 그대로 씁니다.

mod chunk;
mod decode;
mod delta;
mod error;
mod http_date;
pub(crate) mod retry;
mod sse;

pub use decode::StreamDecoder;
pub use delta::{AssembledResponse, DeltaAssembler, StreamEvent, ToolCall, Usage};
pub use error::{FormatErrorKind, StreamParseError};
pub use retry::{
    NotCandidateDetail, PreHeaderConnectKind, RetryDecision, RetryFailureInput, RetryGiveUpReason,
    RetrySettings, decide_retry,
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
