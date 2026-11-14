//! 재시도 판단: 대상 판별, `Retry-After` 해석, Full Jitter·시도 횟수, 판단 반복, 전송 오류 매핑까지 다룹니다.
//!
//! 판별([`classify`])·헤더 대기([`after`])·백오프([`backoff`])·조립([`decide`])·판단 반복([`retry_loop`])·
//! 매핑([`retry_map`])·HTTP-date 읽기([`http_date`])·판단 입력([`input`])을 관심사별로 나눕니다(D25).
//! 판단 반복과 매핑은 전송 쪽에서 이 폴더로 옮겼습니다(작업 08).
//! HTTP-date 읽기도 `codec/http_date.rs`에서 함께 옮겼습니다(03-6 R2 감사 분리).
//! 공개 진입점은 [`decide_retry`]입니다.

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
