//! cancel 시험: 토큰이 끊기면 실행이 멈추고 백그라운드 목록에 남지 않음 확인

use crate::tools::background::shared::lock_background;
use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::approved_context::approved_run_context;
use super::super::args::command_args;
use super::super::shell::long_sleep_command;
use super::super::support::TempFolder;

/// cancel 토큰이 끊기면 명령이 끝나고, 백그라운드 목록에는 남지 않음
#[tokio::test]
async fn cancel_ends_command_without_background_registration() {
    let folder = TempFolder::new("run-cancel-bg");
    let ctx = approved_run_context(&folder, 20);
    ctx.mark_approved();
    ctx.cancel_token().cancel();
    let outcome = RunCommandTool
        .call(&command_args(&long_sleep_command("20")), &ctx)
        .await;
    let ToolOutcome::Error(text) = outcome else {
        panic!("중단 오류여야 합니다: {outcome:?}");
    };
    assert!(text.contains("중단"), "{text}");
    assert!(
        lock_background(&ctx.background).is_empty(),
        "cancel된 명령은 백그라운드 목록에 남지 않음"
    );
}

/// cancel 토큰이 취소되면 실행을 중지하고 결과에 중단을 남김
#[tokio::test]
async fn cancel_token_stops_running_command() {
    let folder = TempFolder::new("run-cancel");
    let ctx = approved_run_context(&folder, 20);
    ctx.mark_approved();
    ctx.cancel_token().cancel();
    let outcome = RunCommandTool
        .call(&command_args(&long_sleep_command("20")), &ctx)
        .await;
    let ToolOutcome::Error(text) = outcome else {
        panic!("중단 오류여야 합니다: {outcome:?}");
    };
    assert!(text.contains("중단"), "{text}");
}
