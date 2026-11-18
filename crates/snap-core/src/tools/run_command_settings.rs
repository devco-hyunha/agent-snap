//! 승인 흐름이 쓰는 시간 상한 값 타입

use std::time::Duration;

use crate::config::AgentSettings;

/// 승인 흐름이 쓰는 두 시간 상한(승인 대기·명령 실행)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunCommandSettings {
    /// 명령 실행 시간 상한. 만료는 미완료로 봄
    pub run_limit: Duration,
    /// 승인 응답을 기다리는 시간 상한. 만료는 거부로만 봄
    pub approval_limit: Duration,
}

impl RunCommandSettings {
    /// 설정에서 읽은 `agent` 섹션 값으로 채움
    #[must_use]
    pub const fn from_settings(settings: &AgentSettings) -> Self {
        Self {
            run_limit: settings.run_command_seconds,
            approval_limit: settings.approval_seconds,
        }
    }

    /// 기획 기본값(실행 600초·승인 300초)으로 맞춤
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::from_settings(&AgentSettings::default())
    }
}

impl Default for RunCommandSettings {
    fn default() -> Self {
        Self::with_defaults()
    }
}
