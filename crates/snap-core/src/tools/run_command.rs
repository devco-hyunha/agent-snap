//! `run_command` 도구: 셸 명령 한 개를 승인받은 뒤에 셸에 넘겨 결과를 받음
//!
//! 승인 전에는 `ApprovalRequired`만 돌려줌. 같은 호출이 `approved` 표식과 함께 오면 그때 셸 명령을 넘김
//!
//! 한계: 명령 문자열 속 작업 폴더 밖 절대 경로·비밀 파일 읽기는 이름 검사로 못 막음

use std::path::Path;

use serde_json::json;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::task::JoinHandle;

use super::args::{parse_tool_args, str_arg};
use super::path_guard::{PathGuardError, is_secret_name};
use super::process_tree::ProcessTree;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

/// `run_command` 도구
pub struct RunCommandTool;

impl Tool for RunCommandTool {
    fn name(&self) -> &str {
        "run_command"
    }

    fn description(&self) -> &str {
        "셸 명령을 실행한다. 승인자가 승인한 명령만 작업 폴더를 기준으로 실행한다"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {"type": "string", "description": "셸에 넘길 명령 문자열"}
            },
            "required": ["command"]
        })
    }

    /// 승인 대상은 `run_command` 한 개. 파일 쓰기는 승인 대상 아님
    fn executes_writes(&self) -> bool {
        true
    }

    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            let args = match parse_tool_args(args_json) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            let command = match str_arg(&args, "command") {
                Ok(command) => command,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            // 비밀 파일 이름을 명령 문자열에서도 보는 보조 검사
            if command_has_secret_name(command) {
                return ToolOutcome::error(PathGuardError::SecretFile.message());
            }
            // 승인 표식은 이 호출에서만 소비함
            if !ctx.take_approved() {
                return ToolOutcome::ApprovalRequired {
                    summary: command.to_owned(),
                };
            }
            run_approved(command, ctx).await
        })
    }
}

/// 승인된 명령만 셸로 실행해 도구 결과로 바꿈
///
/// 표준 출력·표준 오류를 각각 별도 태스크로 읽어 파이프가 막히는 일을 피하고, 상한 만료와 cancel을 `select!`로 함께 듣음
async fn run_approved(command: &str, ctx: &ToolContext) -> ToolOutcome {
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
            let note = append_tree_note("미완료: 실행 시간 상한을 넘겼습니다", &mut tree).await;
            return ToolOutcome::error(&note);
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

/// 명령을 끊는 자리에서 트리를 끝낸 결과를 결과 문구에 붙임
///
/// 트리가 이미 끝나면 종료 요청이 실패하지만 그 경우는 지장이 없음
async fn append_tree_note(text: &str, tree: &mut ProcessTree) -> String {
    let text = text.to_owned();
    match tree.terminate().await {
        Ok(()) => text,
        Err(_) => format!("{text}(트리 종료 표기 실패)"),
    }
}

/// 명령 문자열 안의 비밀 파일 이름 조각 확인 보조 검사
///
/// 셸이 명령을 나누는 문자(`;`·`|`·탭·줄바꿈)까지 잘라 각 조각의 마지막 이름만 비교
fn command_has_secret_name(command: &str) -> bool {
    command
        .split([' ', '"', '\\', '/', ';', '|', '\t', '\n'])
        .any(|piece| is_secret_name(Path::new(piece)))
}

/// 표준 출력·표준 오류 읽기 태스크 생성
fn spawn_pipe_reader<R>(pipe: R) -> JoinHandle<String>
where
    R: AsyncRead + Send + Unpin + 'static,
{
    tokio::spawn(read_pipe(pipe))
}

async fn read_pipe<R>(mut pipe: R) -> String
where
    R: AsyncRead + Unpin,
{
    let mut bytes = Vec::new();
    if pipe.read_to_end(&mut bytes).await.is_err() {
        return String::new();
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// 읽기 태스크 결과를 받음. 태스크가 죽으면 빈 문자열로 봄
async fn collect_pipe(task: Option<JoinHandle<String>>) -> String {
    match task {
        Some(task) => task.await.unwrap_or_default(),
        None => String::new(),
    }
}

/// 종료 코드·표준 출력·표준 오류를 도구 결과 텍스트로 만듦(잘림은 루프가 적용)
fn command_result_text(stdout_text: &str, stderr_text: &str, code: Option<i32>) -> String {
    let code_text = code.map_or("알 수 없음".to_owned(), |code| code.to_string());
    let mut text = format!("종료 코드: {code_text}");
    if !stdout_text.is_empty() {
        text.push_str("\n표준 출력:\n");
        text.push_str(stdout_text);
    }
    if !stderr_text.is_empty() {
        text.push_str("\n표준 오류:\n");
        text.push_str(stderr_text);
    }
    text
}
