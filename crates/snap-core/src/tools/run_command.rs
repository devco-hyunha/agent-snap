//! `run_command` 도구: 셸 명령 한 개를 승인받은 뒤에 셸에 넘겨 결과를 받습니다(03-6 T1, spec AC 1~5, D22).
//!
//! 결론: 승인 흐름은 03-2 `executor`가 정본 창구이고, 이 도구는 승인 전에는 `ApprovalRequired`만
//! 돌려줍니다. 같은 호출이 `approved` 표식과 함께 오면 그때 셸 명령을 넘깁니다(D49: 승인 대상은
//! `run_command` 한 개, 파일 쓰기는 승인 대상 아님).
//! 결과 잘림은 03-2 `truncate`(루프가 결과에 적용)를 그대로 쓰고, 03-3 경로 검사로 작업 폴더
//! 안에서만 돕니다.
//!
//! 프로세스 트리 정리는 03-7 배치라 여기서는 자식만 끝냅니다.
//!
//! 한계(D17·plan.md 리스크): 명령 문자열 속 작업 폴더 밖 절대 경로·비밀 파일 읽기는
//! 03-3 이름 검사로 막지 못합니다. 승인자(L1)가 그 자리를 막는 기본을 그대로 둡니다.

use std::path::Path;
use std::process::Stdio;

use serde_json::json;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::{Child, Command};
use tokio::task::JoinHandle;

use super::args::{parse_tool_args, str_arg};
use super::path_guard::{PathGuardError, is_secret_name};
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

/// `run_command` 도구입니다.
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

    /// L1 승인 대상은 `run_command` 한 개입니다(D22 5.3절·D49: 파일 쓰기는 승인 대상 아님).
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
            // 03-3 `secret_names`의 이름 판정을 명령 문자열에서도 보는 보조 검사입니다(D17).
            // 이름 검사로 못 막는 우회까지 막지는 못합니다. 파일 위 주석에 적힌 한계의 자리입니다.
            if command_has_secret_name(command) {
                return ToolOutcome::error(PathGuardError::SecretFile.message());
            }
            // 승인 표식은 이 호출에서만 소비한다. 승인자(L1)가 승인한 호출을 executor가 채워 넘긴다.
            if !ctx.take_approved() {
                return ToolOutcome::ApprovalRequired {
                    summary: command.to_owned(),
                };
            }
            run_approved(command, ctx).await
        })
    }
}

/// 승인된 명령만 셸로 실행해 도구 결과로 바꿉니다.
///
/// tokio 공식 지침대로 표준 출력·표준 오류를 각각 별도 태스크로 읽어 파이프가 막히는 일을
/// 피하고, 실행 상한과 cancel을 `select!`로 함께 듣습니다. 상한 만료는 `미완료`, cancel은
/// `중단`으로 결과에 남기고 자식을 `kill`합니다(트리 정리는 03-7 배치).
async fn run_approved(command: &str, ctx: &ToolContext) -> ToolOutcome {
    let Ok(mut child) = shell_command(command, &ctx.working_dir) else {
        return ToolOutcome::error("명령을 실행하지 못했습니다");
    };
    let stdout_task = child.stdout.take().map(spawn_pipe_reader);
    let stderr_task = child.stderr.take().map(spawn_pipe_reader);
    let cancel = ctx.cancel_token();
    let status = tokio::select! {
        () = cancel.cancelled() => {
            let note = append_kill_note("중단: 명령을 중지했습니다", &mut child).await;
            return ToolOutcome::error(&note);
        }
        () = tokio::time::sleep(ctx.run_command.run_limit) => {
            let note = append_kill_note("미완료: 실행 시간 상한을 넘겼습니다", &mut child).await;
            return ToolOutcome::error(&note);
        }
        status = child.wait() => status,
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

/// 명령을 끊는 자리에서 자식을 끝낸 결과를 결과 문구에 붙입니다.
///
/// 자식이 이미 끝나면 `kill`이 실패하지만 그 경우는 지장이 없습니다(dev-principles 7절:
/// 실패를 삼키지 않고 표기로 남깁니다). 트리 정리는 03-7 배치라 자식 하나만 끝냅니다.
async fn append_kill_note(text: &str, child: &mut Child) -> String {
    let text = text.to_owned();
    match child.kill().await {
        Ok(()) => text,
        Err(_) => format!("{text}(자식 종료 표기 실패)"),
    }
}

/// 명령 문자열 안에 비밀 파일 이름으로 보이는 조각이 있는지 확인합니다(D17 보조 검사).
///
/// 03-3 `secret_names`의 이름 판정을 그대로 씁니다(dev-principles 3절 DRY). 셸이 명령을
/// 나누는 문자(`;`·`|`·탭·줄바꿈)까지 잘라 각 조각의 마지막 이름만 비교하고, `type ./credentials.jsonc`처럼
/// 경로로 적힌 조각까지 걸러냅니다. 이름으로 잡히지 않는 우회(변수·인코딩·와일드카드)까지 막지는 못합니다.
/// 이 한계는 파일 위 주석에 기록했고 승인자(L1)가 기본 창구입니다.
fn command_has_secret_name(command: &str) -> bool {
    command
        .split([' ', '"', '\\', '/', ';', '|', '\t', '\n'])
        .any(|piece| is_secret_name(Path::new(piece)))
}

/// 표준 출력·표준 오류를 OS 셸에 넘기는 `Command`을 만들어 줍니다(두 OS 동등, 개발 규칙).
///
/// Windows는 `cmd.exe /C`, 그 밖의 운영체제는 `sh -c`를 씁니다. 명령 문자열은 승인 화면에
/// 그대로 보이는 값이고, 승인 없이는 이 함수에 닿지 않습니다(dev-principles 8절: 신뢰할 수
/// 없는 입력을 다루는 자리는 승인자라는 뜻).
#[cfg(windows)]
fn shell_command(command: &str, root: &Path) -> std::io::Result<Child> {
    let mut cmd = Command::new("cmd.exe");
    cmd.arg("/C").arg(command);
    prepare(cmd, root)
}

#[cfg(unix)]
fn shell_command(command: &str, root: &Path) -> std::io::Result<Child> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    prepare(cmd, root)
}

#[cfg(not(any(windows, unix)))]
fn shell_command(_command: &str, _root: &Path) -> std::io::Result<Child> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "이 운영체제에서는 셸 명령을 지원하지 않습니다",
    ))
}

/// 셸 명령 공통 준비(기준 폴더·파이프·드롭 시 kill)를 한 자리에서 채웁니다.
#[cfg(any(windows, unix))]
fn prepare(mut cmd: Command, root: &Path) -> std::io::Result<Child> {
    cmd.current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    cmd.spawn()
}

/// 표준 출력·표준 오류 읽기 태스크를 만드는 한 자리입니다(두 파이프에 같은 함수를 씁니다).
fn spawn_pipe_reader<R>(pipe: R) -> JoinHandle<String>
where
    R: AsyncRead + Send + Unpin + 'static,
{
    tokio::spawn(read_pipe(pipe))
}

/// 파이프 한쪽을 끝까지 문자열로 읽습니다. 읽기 실패는 빈 문자열로 봅니다.
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

/// 읽기 태스크 결과를 받습니다. 태스크가 죽으면 빈 문자열로 봅니다.
async fn collect_pipe(task: Option<JoinHandle<String>>) -> String {
    match task {
        Some(task) => task.await.unwrap_or_default(),
        None => String::new(),
    }
}

/// 종료 코드·표준 출력·표준 오류를 도구 결과 텍스트 덩어리로 만듭니다(잘림은 루프가 적용).
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
