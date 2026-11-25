//! 재시도를 포기하는 이유 타입 자리.

use std::time::Duration;

use crate::llm::retry::failures::FailureInput;

/// 재시도를 포기하는 이유.
///
/// 키·base_url 호스트·응답 본문은 담지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryReason {
    /// 재시도 대상이 아닌 입력. 상태 코드(해당 시) 또는 입력 종류만 담음.
    NotRetryable {
        /// 판단에 쓴 실패 입력.
        input: FailureInput,
    },

    /// 시도 횟수가 최대 재시도 횟수를 넘음.
    AttemptLimit {
        /// 포기로 본 시도 횟수.
        attempt: u64,
    },

    /// `Retry-After`가 상한을 넘음. 헤더가 지시한 대기 시간만 담음.
    RetryAfterCap {
        /// 헤더가 지시한 대기 시간.
        wait: Duration,
    },
}
