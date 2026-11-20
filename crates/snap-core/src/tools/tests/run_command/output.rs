//! 승인된 명령 실행 결과 시험: 표준 출력·종료 코드 기록과 작업 폴더 기준 실행 확인

use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::approved_context::approved_run_context;
use super::super::args::command_args;
use super::super::shell::READ_MARKER_COMMAND;
use super::super::support::TempFolder;

/// 승인 표식이 채워진 뒤에만 셸 명령이 실행되고, 표준 출력과 종료 코드가 결과에 남음
#[tokio::test]
async fn approved_command_runs_and_reports_output() {
    let folder = TempFolder::new("run-ok");
    let ctx = approved_run_context(&folder, 20);
    ctx.mark_approved();
    let outcome = RunCommandTool
        .call(&command_args("echo snap-core-run"), &ctx)
        .await;
    let ToolOutcome::Done(text) = outcome else {
        panic!("승인된 명령은 실행되어야 합니다: {outcome:?}");
    };
    assert!(text.contains("종료 코드: 0"), "{text}");
    assert!(text.contains("snap-core-run"), "표준 출력: {text}");
}

/// 승인된 명령은 작업 폴더를 기준으로 실행됨
///
/// 확인은 `pwd`·`cd` 출력이 아닌 파일 존재로 봄. 경계 밖은 승인자가 막음
#[tokio::test]
async fn approved_command_runs_inside_working_folder() {
    let folder = TempFolder::new("run-workdir");
    folder.write_file("marker.txt", "marker");
    let ctx = approved_run_context(&folder, 20);
    ctx.mark_approved();
    let outcome = RunCommandTool
        .call(&command_args(READ_MARKER_COMMAND), &ctx)
        .await;
    let ToolOutcome::Done(text) = outcome else {
        panic!("실행되어야 합니다: {outcome:?}");
    };
    assert!(text.contains("marker"), "작업 폴더 안 파일 읽기: {text}");
}
