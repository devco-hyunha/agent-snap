//! `write_file` 도구: 새로 쓰기·덮어쓰기(03-3 T1, spec AC 2·AC 4, D49).
//!
//! 경로 검사는 `path_guard`의 쓰기 검사 한 자리만 부르고, 도구마다 검사를 다시
//! 정의하지 않습니다(DRY). 쓰기는 승인 대상이 아니고(D49) 덮어쓰기 사실만 결과로
//! 알립니다. 부모 폴더가 없으면 함께 만들고, `.git` 아래와 비밀 파일은 쓰지 않습니다.

use std::path::Path;

use serde_json::{Value, json};

use super::args::parse_tool_args;
use super::display::relative_path;
use super::path_guard::check_write_path;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

/// `write_file` 도구입니다.
pub struct WriteFileTool;

impl Tool for WriteFileTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "텍스트 파일 쓰기. 새로 쓰기와 덮어쓰기를 구분해 표시한다. 작업 폴더 아래만 쓸 수 있다"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "쓸 파일 경로"},
                "content": {"type": "string", "description": "파일 내용을 그대로 넣을 문자열"}
            },
            "required": ["path", "content"]
        })
    }

    /// 쓰기 도구이므로 03-8 승인 구분에 표식을 남깁니다. L1 승인 대상은 `run_command`만
    /// 대상으로 두고 파일 쓰기는 승인 없이 진행합니다(D49, D22 5.3절).
    fn executes_writes(&self) -> bool {
        true
    }

    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            let args = match parse_tool_args(args_json) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            let Some(path) = args.get("path").and_then(Value::as_str) else {
                return ToolOutcome::error("path는 문자열이어야 합니다");
            };
            let Some(content) = args.get("content").and_then(Value::as_str) else {
                return ToolOutcome::error("content는 문자열이어야 합니다");
            };
            // 쓰기 검사(경계·비밀·`.git`)를 이 자리 한 곳에서만 부릅니다.
            let target = match check_write_path(&ctx.working_dir, path) {
                Ok(target) => target,
                Err(guard) => return ToolOutcome::error(guard.message()),
            };
            write_text(&target, &ctx.working_dir, content)
        })
    }
}

/// 파일에 내용을 쓰고 성공 표시를 결과로 만들어 줍니다.
///
/// 파일이 있으면 덮어쓴다고, 없으면 새로 썼다고 알립니다. 부모 폴더가 없으면 함께
/// 만듭니다. 쓰기 실패는 이유 한 줄로만 남깁니다(AC 4: 비밀 값·경로 값을 표시에 넣지 않음).
fn write_text(target: &Path, working_dir: &Path, content: &str) -> ToolOutcome {
    let existed = target.exists();
    // 부모 폴더가 없으면 함께 만듭니다. 만들지 못하면 결과를 자르는 자리에서 early return으로
    // 끊고 쓰기 단계로 넘어가지 않습니다(clippy collapsible_if 대응).
    let Some(parent) = target.parent() else {
        return ToolOutcome::error("파일을 쓰지 못했습니다");
    };
    if std::fs::create_dir_all(parent).is_err() {
        return ToolOutcome::error("파일을 쓰지 못했습니다");
    }
    match std::fs::write(target, content) {
        Ok(()) => {
            let mode = if existed {
                "덮어쓰기"
            } else {
                "새로 쓰기"
            };
            let display = relative_path(target, working_dir);
            ToolOutcome::Done(format!("{mode}: {display}"))
        }
        Err(_) => ToolOutcome::error("파일을 쓰지 못했습니다"),
    }
}
