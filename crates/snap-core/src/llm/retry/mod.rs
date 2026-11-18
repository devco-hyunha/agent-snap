//! 재시도 판단. 대상 판별·`Retry-After`·Full Jitter·시도 횟수·판단 반복·전송 오류 매핑.
//!
//! 공개 진입점은 [`decide_retry`].

pub(crate) mod after;
pub(crate) mod backoff;
pub(crate) mod classify;
mod decide;
pub(in crate::llm) mod http_date;
pub(in crate::llm) mod input;
pub(crate) mod retry_loop;
pub(crate) mod retry_map;

pub use decide::{
    NotCandidateDetail, RetryDecision, RetryGiveUpReason, RetrySettings, decide_retry,
};
pub use input::{PreHeaderConnectKind, RetryFailureInput};
