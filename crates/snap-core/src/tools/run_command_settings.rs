//! 03-6 승인 흐름이 쓰는 시간 상한 값 타입만 둡니다(03-6 T1, SRP).
//!
//! 설정에서 읽은 [`AgentSettings`]를 `Duration` 두 값으로 바꾸는 자리만 맡습니다.
//! 승인 대기·실행 판단은 `approval/approver.rs`가, 명령 실행은 `run_command.rs`가 씁니다.

use std::time::Duration;

use crate::config::AgentSettings;

/// 03-6 승인 흐름이 쓰는 두 시간 상한(승인 대기·명령 실행)입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunCommandSettings {
    /// 명령 실행 시간 상한입니다. 만료는 미완료로 봅니다(D22).
    pub run_limit: Duration,
    /// 승인 응답을 기다리는 시간 상한입니다. 만료는 거부로만 봅니다(D22).
    pub approval_limit: Duration,
}

impl RunCommandSettings {
    /// 설정에서 읽은 `agent` 섹션 값으로 채웁니다(spec AC 3: 02 설정 읽기 경로 재사용).
    #[must_use]
    pub const fn from_settings(settings: &AgentSettings) -> Self {
        Self {
            run_limit: settings.run_command_seconds,
            approval_limit: settings.approval_seconds,
        }
    }

    /// 기획 기본값(D22: 실행 600초·승인 300초)으로 맞춥니다.
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
