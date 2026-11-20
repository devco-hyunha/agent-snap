//! 실행 상한 만료 → 백그라운드 등록 → wake 완료 왕복 시험
//!
//! wake 주기는 짧게 주고, 왕복은 wake를 반복해 완료만 확인

use std::time::Duration;

use crate::tools::background::shared::lock_background;
use crate::tools::check_background::CheckBackgroundTool;
use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::approved_context::approved_run_context;
use super::super::args::command_args;
use super::super::shell::sleep_then_echo;
use super::super::support::TempFolder;
use super::wake_helpers::wake_until_finished;

/// 실행 시간 상한 만료 → 백그라운드 등록 → wake 완료로 이어지는 왕복
#[tokio::test]
async fn deadline_moves_to_background_and_wake_reports_completion() {
    let folder = TempFolder::new("bg-roundtrip");
    let mut ctx = approved_run_context(&folder, 1);
    ctx.wake_interval = Duration::from_millis(500);
    ctx.mark_approved();
    let outcome = RunCommandTool
        .call(&command_args(&sleep_then_echo("snap-bg-mark")), &ctx)
        .await;
    let ToolOutcome::Done(transition) = outcome else {
        panic!("백그라운드 표시여야 합니다: {outcome:?}");
    };
    assert!(
        transition.contains("백그라운드로 옮겼습니다"),
        "{transition}"
    );
    assert_eq!(
        lock_background(&ctx.background).len(),
        1,
        "만료된 명령이 백그라운드 목록에 등록되어야 합니다"
    );

    let text = wake_until_finished(&ctx, &CheckBackgroundTool).await;
    assert!(text.contains("완료: 종료 코드"), "{text}");
    assert!(
        text.contains("snap-bg-mark"),
        "출력이 wake 결과에 붙어야 합니다: {text}"
    );
    assert!(
        lock_background(&ctx.background).is_empty(),
        "완료된 명령은 백그라운드 목록에서 빠져야 합니다"
    );
}
