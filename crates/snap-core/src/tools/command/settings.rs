//! 명령 시간 상한 값 타입
//!
//! 승인 대기 상한은 `approval/settings`가 따로 봄

use std::time::Duration;

use crate::config::AgentSettings;

/// 명령 상한 값
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunCommandSettings {
    /// 명령 상한. 만료는 백그라운드로 옮김
    pub run_limit: Duration,
}

impl RunCommandSettings {
    /// 설정에서 읽은 `agent` 섹션 값으로 채움
    #[must_use]
    pub const fn from_settings(settings: &AgentSettings) -> Self {
        Self {
            run_limit: settings.run_command_seconds,
        }
    }

    /// 기획 기본값(600초)으로 맞춤
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
