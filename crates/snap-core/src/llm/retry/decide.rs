//! 재시도 판단 조립 자리. 판정 순서는 결정 12번: 대상 판별 → 시도 횟수 초과 → `Retry-After` 해석 → 백오프.
//!
//! 실제 대기는 하지 않고 판단만 돌려줌. 난수와 현재 시각은 주입받음.

use std::time::Duration;
use std::time::SystemTime;

use crate::llm::retry::after::interpret_retry_after;
use crate::llm::retry::backoff::full_jitter_wait;
use crate::llm::retry::backoff::is_attempt_limit_exceeded;
use crate::llm::retry::classify::is_candidate;
use crate::llm::retry::decision::RetryDecision;
use crate::llm::retry::failures::FailureInput;
use crate::llm::retry::failures::RetryCandidate;
use crate::llm::retry::outcome::RetryAfterOutcome;
use crate::llm::retry::random::RandomSource;
use crate::llm::retry::reason::RetryReason;
use crate::llm::retry::settings::RetrySettings;

/// 재시도할지와 대기 시간을 한 번에 판단함.
///
/// 시도 횟수 초과는 `Retry-After` 상한 초과보다 먼저 본다(결정 12번).
pub fn decide_retry(
    input: FailureInput,
    retry_after: Option<&str>,
    attempt: u64,
    settings: &RetrySettings,
    now: SystemTime,
    rng: &mut dyn RandomSource,
) -> RetryDecision {
    if is_candidate(input) == RetryCandidate::No {
        return RetryDecision::GiveUp {
            reason: RetryReason::NotRetryable { input },
        };
    }
    if is_attempt_limit_exceeded(attempt, settings.max_retries) {
        return RetryDecision::GiveUp {
            reason: RetryReason::AttemptLimit { attempt },
        };
    }
    let cap = Duration::from_secs(settings.retry_after_max);
    match interpret_retry_after(retry_after, now, cap) {
        RetryAfterOutcome::Wait(wait) => RetryDecision::Wait { wait },
        RetryAfterOutcome::ExceedsCap { wait } => RetryDecision::GiveUp {
            reason: RetryReason::RetryAfterCap { wait },
        },
        RetryAfterOutcome::Absent => RetryDecision::Wait {
            wait: full_jitter_wait(attempt, settings, rng),
        },
    }
}
