//! 실행 상한 만료 시험: 상한을 넘기면 백그라운드로 옮긴 표시만 넘김 확인

use std::time::Duration;

use crate::tools::background::shared::lock_background;
use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::approved_context::approved_run_context;
use super::super::args::command_args;
use super::super::shell::long_sleep_command;
use super::super::support::TempFolder;

/// 실행 상한(짧게 주입)을 넘기면 백그라운드로 옮겼다는 표시 한 줄만 넘김
#[tokio::test]
async fn run_deadline_moves_to_background() {
    let folder = TempFolder::new("run-deadline");
    let ctx = approved_run_context(&folder, 1);
    ctx.mark_approved();
    let started = std::time::Instant::now();
    let outcome = RunCommandTool
        .call(&command_args(&long_sleep_command("20")), &ctx)
        .await;
    let ToolOutcome::Done(text) = outcome else {
        panic!("백그라운드 표시여야 합니다: {outcome:?}");
    };
    assert!(text.contains("백그라운드로 옮겼습니다"), "{text}");
    assert_eq!(text.lines().count(), 1, "표시 한 줄만: {text}");
    assert!(
        lock_background(&ctx.background).len() == 1,
        "백그라운드 목록에 등록되어야 합니다"
    );
    assert!(
        started.elapsed() < Duration::from_secs(12),
        "실행 상한(1초) 안에 표시로 돌아와야 합니다: {:?}",
        started.elapsed()
    );
}
