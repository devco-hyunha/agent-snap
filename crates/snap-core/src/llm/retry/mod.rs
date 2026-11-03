//! 재시도 판단: 대상 판별, `Retry-After` 해석, Full Jitter·시도 횟수, 공개 진입점입니다.
//!
//! 판별([`classify`])·헤더 대기([`after`])·백오프([`backoff`])·조립([`decide`])을
//! 관심사별로 나눕니다(D25). 공개 진입점은 [`decide_retry`]입니다.

pub(crate) mod after;
pub(crate) mod backoff;
pub(crate) mod classify;
mod decide;

pub use classify::{PreHeaderConnectKind, RetryFailureInput};
pub use decide::{
    NotCandidateDetail, RetryDecision, RetryGiveUpReason, RetrySettings, decide_retry,
};
