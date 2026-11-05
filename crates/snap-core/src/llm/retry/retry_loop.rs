//! 전송이 실패한 한 건을 02-4 판단으로 돌려 재시도·포기를 고르는 "판단 반복" 책임입니다.
//!
//! 재시도 규칙은 여기서 정의하지 않고 [`decide_retry`]만 부릅니다(02-6 설계 요약).
//! 대기 sleep과 취소 경합은 전송(`transport/stream_chat.rs`)이 맡고, 이 파일은 판단 결과만 돌려줍니다.
//! `Retry-After` 헤더 원문은 판단 입력으로만 쓰고 공개 오류에 담지 않습니다(02-6 spec 33행).

use std::time::{Duration, SystemTime};

use crate::llm::client::TransportError;
use crate::llm::retry::{RetryDecision, RetryGiveUpReason, RetrySettings, decide_retry};

use super::retry_map::retry_input;

/// 판단 반복 한 번의 결과입니다.
#[derive(Debug)]
pub(crate) enum LoopOutcome {
    /// 02-4가 정한 대기 시간만큼 기다린 뒤 다시 시도하라는 권고입니다(0초 포함).
    Retry {
        /// 02-4가 정한 대기 시간입니다.
        wait: Duration,
    },
    /// 포기가 확정된 값입니다. 호출부에 돌려줄 최종 전송 오류입니다.
    GiveUp(TransportError),
}

/// 실패한 시도 한 건을 02-4 판단에 넣어 재시도·포기를 고릅니다.
///
/// `attempt`는 방금 실패한 요청의 시도 번호입니다(첫 요청=1).
/// `retry_after`은 응답이 준 `Retry-After` 헤더 원문입니다.
/// `unit_sample`은 Full Jitter용 `[0.0, 1.0]` 균등 난수이고, `None`이면 생산 공급(`fastrand`)을 씁니다.
/// 테스트는 값을 주입해 대기 시간을 고정합니다(02-4 주입 패턴).
pub(crate) fn judge_once(
    error: TransportError,
    retry_after: Option<&str>,
    attempt: u32,
    settings: RetrySettings,
    now: SystemTime,
    unit_sample: Option<f64>,
) -> LoopOutcome {
    let Some(input) = retry_input(&error, retry_after.is_some()) else {
        // 판단 대상이 아닌 실패(취소·스트림 중 오류)는 판단 없이 그대로 넘깁니다.
        return LoopOutcome::GiveUp(error);
    };
    let unit_sample = unit_sample.unwrap_or(fastrand::f64());
    match decide_retry(input, retry_after, attempt, settings, now, unit_sample) {
        RetryDecision::Retry { wait } => LoopOutcome::Retry { wait },
        RetryDecision::GiveUp(reason) => LoopOutcome::GiveUp(give_up_error(error, reason)),
    }
}

/// 포기가 정해진 마지막 오류를 최종 오류로 바꿉니다. 서버 상태 응답에만 02-4 포기 이유를 얹고
/// (새 오류 종류 금지, spec 33행) 그 밖의 실패는 그대로 둡니다.
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
