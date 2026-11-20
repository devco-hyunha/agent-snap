//! 승인 표식 시험: 승인 전에는 셸에 넘기지 않고, 표식은 그 호출에서만 소비됨 확인

use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::approved_context::approved_run_context;
use super::super::args::command_args;
use super::super::support::{TempFolder, context_of};

/// 승인 표식이 없는 호출은 셸에 넘기지 않고 승인 요청만 반환(미실행)
#[tokio::test]
async fn call_without_approval_returns_approval_required() {
    let folder = TempFolder::new("run-noapproval");
    let ctx = context_of(&folder);
    let outcome = RunCommandTool
        .call(&command_args("echo snap-core-run"), &ctx)
        .await;
    assert!(
        matches!(outcome, ToolOutcome::ApprovalRequired { .. }),
        "승인 전에는 승인 요청만: {outcome:?}"
    );
    assert_eq!(RunCommandTool.name(), "run_command");
    assert!(RunCommandTool.executes_writes());
}

/// 승인 표식은 그 호출에서만 쓰임. 같은 판으로 두 번째 호출을 하면 다시 승인 요청만 반환
#[tokio::test]
async fn approval_flag_is_consumed_by_that_call() {
    let folder = TempFolder::new("run-flag-once");
    let ctx = approved_run_context(&folder, 20);
    ctx.mark_approved();
    let ToolOutcome::Done(first) = RunCommandTool
        .call(&command_args("echo snap-first"), &ctx)
        .await
    else {
        panic!("승인된 첫 호출은 실행되어야 합니다");
    };
    assert!(first.contains("snap-first"), "{first}");
    // 표식을 소비했으므로 같은 판에서의 둘째 호출은 셸에 넘기지 않고 승인 요청만 돌려줌
    let outcome = RunCommandTool
        .call(&command_args("echo snap-second"), &ctx)
        .await;
    assert!(
        matches!(outcome, ToolOutcome::ApprovalRequired { .. }),
        "둘째 호출은 승인자를 다시 타야 합니다: {outcome:?}"
    );
}
