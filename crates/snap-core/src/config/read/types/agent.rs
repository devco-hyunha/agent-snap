//! `agent` 섹션 설정 값 타입.

use std::time::Duration;

use super::{
    DEFAULT_APPROVAL_DEADLINE_SECONDS, DEFAULT_RUN_COMMAND_DEADLINE_SECONDS,
    DEFAULT_WAKE_INTERVAL_SECONDS,
};

/// 에이전트 루프가 쓰는 시간 값.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentSettings {
    /// `run_command` 실행 시간 상한. 기본 600초.
    pub run_command_seconds: Duration,
    /// 승인 응답 대기 시간 상한. 기본 300초.
    pub approval_seconds: Duration,
    /// 백그라운드 확인 주기. 기본 30초.
    pub wake_interval_seconds: Duration,
}

impl Default for AgentSettings {
    fn default() -> Self {
        Self {
            run_command_seconds: Duration::from_secs(DEFAULT_RUN_COMMAND_DEADLINE_SECONDS),
            approval_seconds: Duration::from_secs(DEFAULT_APPROVAL_DEADLINE_SECONDS),
            wake_interval_seconds: Duration::from_secs(DEFAULT_WAKE_INTERVAL_SECONDS),
        }
    }
}
