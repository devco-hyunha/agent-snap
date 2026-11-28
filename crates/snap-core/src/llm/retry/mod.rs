//! 재시도 판단 자리. 입력 타입·설정 타입·난수 공급 타입·포기 이유·최종 판단·대상 판별·`Retry-After` 해석·백오프와 시도 횟수 계산·판단 조립·상용 난수원 산술을 책임별 파일로 나눔.

mod after;
mod backoff;
mod calendar;
mod classify;
mod decide;
mod decision;
mod failures;
mod http_date;
mod mixing;
mod outcome;
mod parsing;
mod random;
mod reason;
mod settings;
mod system_random;

pub use after::interpret_retry_after;
pub use backoff::backoff_cap;
pub use backoff::full_jitter_wait;
pub use backoff::is_attempt_limit_exceeded;
pub use classify::is_candidate;
pub use decide::decide_retry;
pub use decision::RetryDecision;
pub use failures::FailureInput;
pub use failures::PreHeaderConnectKind;
pub use failures::RetryCandidate;
pub use outcome::RetryAfterOutcome;
pub use random::RandomSource;
pub use reason::RetryReason;
pub use settings::RetrySettings;
pub use system_random::SystemRandomSource;
