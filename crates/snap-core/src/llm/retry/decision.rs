//! 재시도 최종 판단 타입 자리.

use std::time::Duration;

use crate::llm::retry::reason::RetryReason;

/// 재시도 판단 결과.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryDecision {
    /// 이 시간만큼 기다린 뒤 다시 요청함.
    Wait {
        /// 기다릴 시간.
        wait: Duration,
    },

    /// 포기. 이유를 밝힘.
    GiveUp {
        /// 포기를 결정한 이유.
        reason: RetryReason,
    },
}
