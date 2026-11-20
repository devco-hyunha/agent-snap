//! 백그라운드 목록 드롭(프로그램 종료) 시험: 목록이 묶여서 트리 전체가 정리되는지 확인

use std::time::Duration;

use crate::tools::background::shared::lock_background;
use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::approved_context::approved_run_context;
use super::super::args::command_args;
use super::super::shell::grandchild_marker_command;
use super::super::support::TempFolder;

/// 백그라운드 명령이 프로그램 종료(목록 드롭)로 묶음 정리를 밟는지 확인
#[tokio::test]
async fn registry_drop_ends_background_tree_when_list_drops() {
    let folder = TempFolder::new("bg-drop");
    let marker = folder.path().join("leak.txt");
    {
        let mut ctx = approved_run_context(&folder, 1);
        ctx.wake_interval = Duration::from_millis(300);
        ctx.mark_approved();
        let outcome = RunCommandTool
            .call(&command_args(&grandchild_marker_command("leak.txt")), &ctx)
            .await;
        assert!(
            matches!(outcome, ToolOutcome::Done(_)),
            "백그라운드 표시여야 합니다: {outcome:?}"
        );
        assert_eq!(
            lock_background(&ctx.background).len(),
            1,
            "만료된 명령이 백그라운드 목록에 등록되어야 합니다"
        );
    }
    tokio::time::sleep(Duration::from_secs(5)).await;
    assert!(
        !marker.exists(),
        "손자까지 트리가 끝나 마커가 생기면 안 됩니다"
    );
}
