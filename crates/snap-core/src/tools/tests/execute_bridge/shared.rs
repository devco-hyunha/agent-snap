//! execute 브리지 시험 재사용 재료: `execute` 부르는 창구 한 자리.
//!
//! 승인 표식 채우기 → 호출 → 지우기 순서를 시험에서 반복하지 않게 한 정의로 둠.

#![allow(dead_code)]

use std::time::Duration;

use crate::config::AgentSettings;
use crate::tools::execute::ExecuteTool;
use crate::tools::run_command_settings::RunCommandSettings;
use crate::tools::tool::{Tool, ToolContext};

use super::super::support::TempWorkdir;

/// 승인 표식을 채운 실행 판으로 `execute`를 불러 결과 글자를 받아옴.
pub async fn run_execute(workdir: &TempWorkdir, script: &str) -> String {
    execute_text(&workdir.context(), script).await
}

/// 실행 판을 받아 `execute`를 부르는 한 자리.
pub async fn execute_text(ctx: &ToolContext, script: &str) -> String {
    ctx.mark_approved();
    let args = serde_json::json!({"script": script}).to_string();
    let outcome = ExecuteTool.call(&args, ctx).await;
    ctx.clear_approved();
    outcome.text().to_owned()
}

/// 시험이 쓰는 짧은 명령 실행 상한을 채운 실행 판을 만듦.
#[must_use]
pub fn short_limit_context(workdir: &TempWorkdir, run_limit_ms: u64) -> ToolContext {
    let mut ctx = workdir.context();
    ctx.run_command = RunCommandSettings::from_settings(&AgentSettings {
        run_command_seconds: Duration::from_millis(run_limit_ms),
        approval_seconds: Duration::from_secs(5),
    });
    ctx
}
