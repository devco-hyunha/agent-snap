//! 승인된 셸 명령을 실행 판에서 실행해 도구 결과로 바꾸는 자리
//!
//! 승인 표식을 통과한 명령만 실행함

use crate::tools::background::shared::lock_background;
use crate::tools::process_tree::ProcessTree;
use crate::tools::tool::ToolContext;
use crate::tools::tool::ToolOutcome;

use super::pipes::collect_pipe;
use super::pipes::spawn_pipe_reader;
use super::result_text::command_result_text;
use super::tree_note::append_tree_note;

/// 승인된 명령만 셸로 실행해 도구 결과로 바꿈
///
/// 표준 출력·표준 오류를 각각 별도 태스크로 읽어 파이프가 막히는 일을 피하고, 상한 만료와 cancel을 `select!`로 함께 듣음
pub async fn run_approved(command: &str, ctx: &ToolContext) -> ToolOutcome {
    let Ok(mut tree) = ProcessTree::start(command, &ctx.working_dir) else {
        return ToolOutcome::error("명령을 실행하지 못했습니다");
    };
    let (stdout, stderr) = tree.take_pipes();
    let stdout_task = stdout.map(spawn_pipe_reader);
    let stderr_task = stderr.map(spawn_pipe_reader);
    let cancel = ctx.cancel_token();
    let status = tokio::select! {
        () = cancel.cancelled() => {
            let note = append_tree_note("중단: 명령을 중지했습니다", &mut tree).await;
            return ToolOutcome::error(&note);
        }
        () = tokio::time::sleep(ctx.run_command.run_limit) => {
            // 백그라운드 목록 창구에 트리와 읽기 태스크를 넘기고 옮겼다는 표시 한 줄만 넘김
            let text = lock_background(&ctx.background).register(tree, stdout_task, stderr_task);
            return ToolOutcome::Done(text);
        }
        status = tree.wait() => status,
    };
    let stdout_text = collect_pipe(stdout_task).await;
    let stderr_text = collect_pipe(stderr_task).await;
    match status {
        Ok(status) => ToolOutcome::Done(command_result_text(
            &stdout_text,
            &stderr_text,
            status.code(),
        )),
        Err(_) => ToolOutcome::error("명령 결과를 읽지 못했습니다"),
    }
}
