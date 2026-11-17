//! 03-8 execute 브리지 시험 재사용 재료: `execute` 부르는 창구 한 자리만 둡니다.
//!
//! 03-2 `run_approved_call`과 같은 순서(표식 채우기 → 호출 → 지우기)로 `execute`를 부릅니다.
//! 시간 값은 짧게 주고, 엔진은 전용 스레드에서 굴립니다(테스트 계획).

#![allow(dead_code)]

use std::time::Duration;

use crate::config::AgentSettings;
use crate::tools::execute::ExecuteTool;
use crate::tools::run_command_settings::RunCommandSettings;
use crate::tools::tool::{Tool, ToolContext};

use super::super::support::TempWorkdir;

/// 승인 표식을 채운 실행 판으로 `execute`를 불러 결과 글자를 받아옵니다(AC 1·AC 4).
pub async fn run_execute(workdir: &TempWorkdir, script: &str) -> String {
    execute_text(&workdir.context(), script).await
}

/// 실행 판을 받아 `execute`를 부르는 한 자리입니다(승인 흐름 재사용, DRY).
pub async fn execute_text(ctx: &ToolContext, script: &str) -> String {
    ctx.mark_approved();
    let args = serde_json::json!({"script": script}).to_string();
    let outcome = ExecuteTool.call(&args, ctx).await;
    ctx.clear_approved();
    outcome.text().to_owned()
}

/// 시험이 쓰는 짧은 명령 실행 상한을 채운 실행 판을 만듭니다(AC 6: 03-6 값 재사용 확인).
#[must_use]
pub fn short_limit_context(workdir: &TempWorkdir, run_limit_ms: u64) -> ToolContext {
    let mut ctx = workdir.context();
    ctx.run_command = RunCommandSettings::from_settings(&AgentSettings {
        run_command_seconds: Duration::from_millis(run_limit_ms),
        approval_seconds: Duration::from_secs(5),
    });
    ctx
}
