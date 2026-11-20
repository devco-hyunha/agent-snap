//! `run_command` 도구 창구: 인자·비밀 이름 검사·승인 요청만 담당
//!
//! 승인 표식은 이 호출 한 개에서만 소비하고 만료 표식은 건드리지 않음

use serde_json::json;

use super::approve::run_approved;
use super::secret_name::command_has_secret_name;
use crate::tools::args::{parse_tool_args, str_arg};
use crate::tools::path_guard::PathGuardError;
use crate::tools::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

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
            // 비밀 파일 이름은 셸이 명령을 나누는 문자까지 잘라 확인하는 보조 검사
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
