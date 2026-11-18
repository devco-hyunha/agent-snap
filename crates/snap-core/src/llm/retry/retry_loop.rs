//! 실패한 한 건을 재시도 판단으로 돌려 재시도·포기를 고르는 판단 반복.
//!
//! 규칙은 [`decide_retry`]만 부름. sleep·취소는 전송이 맡고 이 파일은 판단 결과만 반환.

use std::time::{Duration, SystemTime};

use crate::llm::client::TransportError;
use crate::llm::retry::{RetryDecision, RetryGiveUpReason, RetrySettings, decide_retry};

use super::retry_map::retry_input;

/// 판단 반복 한 번의 결과.
#[derive(Debug)]
pub(crate) enum LoopOutcome {
    /// 정한 대기만큼 기다린 뒤 다시 시도하라는 권고(0초 포함).
    Retry {
        /// 정한 대기 시간.
        wait: Duration,
    },
    /// 포기가 확정된 최종 전송 오류.
    GiveUp(TransportError),
}

/// 실패한 시도 한 건을 재시도 판단에 넣어 재시도·포기를 고름.
///
/// `attempt`는 방금 실패한 요청의 시도 번호(첫 요청=1). `unit_sample`이 `None`이면 생산 난수.
pub(crate) fn judge_once(
    error: TransportError,
    retry_after: Option<&str>,
    attempt: u32,
    settings: RetrySettings,
    now: SystemTime,
    unit_sample: Option<f64>,
) -> LoopOutcome {
    let Some(input) = retry_input(&error, retry_after.is_some()) else {
        // 판단 대상이 아닌 실패(취소·스트림 중 오류)는 판단 없이 그대로 넘김.
        return LoopOutcome::GiveUp(error);
    };
    let unit_sample = unit_sample.unwrap_or(fastrand::f64());
    match decide_retry(input, retry_after, attempt, settings, now, unit_sample) {
        RetryDecision::Retry { wait } => LoopOutcome::Retry { wait },
        RetryDecision::GiveUp(reason) => LoopOutcome::GiveUp(give_up_error(error, reason)),
    }
}

/// 포기 확정 오류를 최종 오류로 바꿈. 서버 상태에만 포기 이유를 얹고 나머지는 그대로 둠.
fn give_up_error(error: TransportError, reason: RetryGiveUpReason) -> TransportError {
    match error {
        TransportError::ServerStatus {
            status,
            body_summary,
            ..
        } => TransportError::ServerStatus {
            status,
            body_summary,
            give_up: Some(reason),
        },
        other => other,
    }
}
