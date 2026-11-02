//! 재시도 판단: 대상 판별, `Retry-After` 해석, Full Jitter·시도 횟수, 공개 facade입니다.
//!
//! 판별([`classify`])·헤더 대기([`after`])·백오프([`backoff`])·조립([`decide`])을
//! 관심사별로 나눕니다(D25). 공개 진입점은 [`decide_retry`]입니다.

mod after;
mod backoff;
mod classify;
mod decide;

pub use after::{RetryAfterDecision, interpret_retry_after};
pub use backoff::{
    BackoffDecision, GiveUpPriority, capped_exponential_ceiling, decide_backoff, full_jitter_wait,
    is_attempts_exceeded, resolve_give_up_priority,
};
pub use classify::{PreHeaderConnectKind, RetryClass, RetryFailureInput, classify_retry};
pub use decide::{
    NotCandidateDetail, RetryDecision, RetryGiveUpReason, RetrySettings, decide_retry,
};
