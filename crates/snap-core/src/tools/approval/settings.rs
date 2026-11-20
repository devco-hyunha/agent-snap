//! 승인 대기 시간 상한 값 타입
//!
//! 승인 흐름이 기다리는 시간만 담고, 명령 상한은 `command/settings`가 따로 봄

use std::time::Duration;

use crate::config::AgentSettings;

/// 승인 응답을 기다리는 시간 상한. 만료는 거부로만 봄
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApprovalSettings {
    /// 승인 응답을 기다리는 시간 상한
    pub limit: Duration,
}

impl ApprovalSettings {
    /// 설정에서 읽은 `agent` 섹션 값으로 채움
    #[must_use]
    pub const fn from_settings(settings: &AgentSettings) -> Self {
        Self {
            limit: settings.approval_seconds,
        }
    }

    /// 기획 기본값(300초)으로 맞춤
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::from_settings(&AgentSettings::default())
    }
}

impl Default for ApprovalSettings {
    fn default() -> Self {
        Self::with_defaults()
    }
}
