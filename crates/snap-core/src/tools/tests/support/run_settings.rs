//! 시험용 실행 상한 재료: 실행 상한 초만 짧게 지정한 설정

use std::time::Duration;

use crate::config::AgentSettings;
use crate::tools::command::settings::RunCommandSettings;

/// 상한을 짧게 지정한 설정(실행 상한 초)
#[must_use]
pub fn run_settings(run_seconds: u64) -> RunCommandSettings {
    RunCommandSettings::from_settings(&AgentSettings {
        run_command_seconds: Duration::from_secs(run_seconds),
        ..AgentSettings::default()
    })
}
