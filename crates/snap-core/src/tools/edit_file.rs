//! `edit_file` 도구: 정확한 문자열 교체(03-3 T1, spec AC 3).
//!
//! 경로는 `path_guard`를 호출만 하고, 일치가 하나일 때만 바꿉니다. 일치 없음·다중 일치는
//! 짧은 오류 한 줄로 알립니다(SWE-agent 2405.15793의 편집 최소·짧은 오류 교훈).
//! 쓰기는 승인 대상이 아닙니다(D49).

use std::path::Path;

use serde_json::{Value, json};

use super::args::parse_tool_args;
use super::display::relative_path;
use super::path_guard::check_write_path;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

/// `edit_file` 도구입니다.
pub struct EditFileTool;

impl Tool for EditFileTool {
    fn name(&self) -> &str {
        "edit_file"
    }

    fn description(&self) -> &str {
        "텍스트 파일에서 찾을 문자열을 정확히 하나만 바꿔 쓴다. 일치가 없거나 둘 이상이면 실패한다"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "고칠 파일 경로"
                },
                "find": {
                    "type": "string",
                    "description": "찾을 문자열"
                },
                "replace": {
                    "type": "string",
                    "description": "바꿀 문자열"
                }
            },
            "required": ["path", "find", "replace"]
        })
    }

    /// 파일 쓰기는 03-8 승인 구분 기준입니다. L1 승인 대상은 `run_command`만 해당합니다(D49).
    fn executes_writes(&self) -> bool {
        true
    }

    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            let value = match parse_tool_args(args_json) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            let path = match value.get("path").and_then(Value::as_str) {
                Some(path) => path,
                None => return ToolOutcome::error("path는 문자열이어야 합니다"),
            };
            let find = match value.get("find").and_then(Value::as_str) {
                Some(find) => find,
                None => return ToolOutcome::error("find는 문자열이어야 합니다"),
            };
            let Some(replace) = value.get("replace").and_then(Value::as_str) else {
                return ToolOutcome::error("replace는 문자열이어야 합니다");
            };
            let target = match check_write_path(&ctx.working_dir, path) {
                Ok(target) => target,
                Err(guard) => return ToolOutcome::error(guard.message()),
            };
            replace_once(&target, &ctx.working_dir, find, replace)
        })
    }
}

/// 파일을 읽고 찾을 문자열을 한 곳에서만 바꿔 씁니다(spec AC 3).
///
/// 바꿀 문자열이 찾을 문자열과 같아도 일치가 하나면 성공이고 파일은 그대로입니다.
/// 일치가 없거나 둘 이상이면 파일을 고치지 않고 이유 한 줄만 돌려줍니다.
fn replace_once(target: &Path, working_dir: &Path, find: &str, replace: &str) -> ToolOutcome {
    let Ok(text) = std::fs::read_to_string(target) else {
        return ToolOutcome::error("파일을 읽지 못했습니다");
    };
    let found = text.match_indices(find).count();
    match found {
        0 => ToolOutcome::error("일치 없음"),
        1 => {
            let edited = text.replace(find, replace);
            match std::fs::write(target, &edited) {
                Ok(()) => ToolOutcome::Done(edit_message(target, working_dir, find, replace)),
                Err(_) => ToolOutcome::error("파일을 고치지 못했습니다"),
            }
        }
        many => ToolOutcome::error(&format!("다중 일치({many}곳)")),
    }
}

/// 교체 결과 표시 한 줄 문장입니다. 바꿀 문자열이 찾을 문자열과 같으면 그 사실을 더 적습니다.
fn edit_message(target: &Path, working_dir: &Path, find: &str, replace: &str) -> String {
    let same = if find == replace { "(같음)" } else { "" };
    let space = if same.is_empty() { "" } else { " " };
    format!(
        "교체 성공: {} (1곳{space}{same})",
        relative_path(target, working_dir)
    )
}
