//! `list_dir` 도구: 작업 폴더 아래 자리를 나열합니다(03-3 T2).
//!
//! 숨김 자리는 기본 제외(opt로 포함), 비밀 파일 자리는 늘 제외합니다.
//! 경계 검사와 순회는 `walk`의 자리 모음이 하고, 이 파일은 표시만 맡습니다(책임 분리).

use serde_json::{Value, json};

use super::args::{bool_arg, opt_str, parse_tool_args};
use super::display::cap_lines;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};
use super::walk::{collect, resolve_dir};

/// `list_dir` 도구입니다.
pub struct ListDirTool;

impl ListDirTool {
    const NAME: &'static str = "list_dir";
}

impl Tool for ListDirTool {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        "작업 폴더 아래 자리 목록. 숨김은 기본 제외(opt로 포함), 비밀 파일은 제외, 재귀 opt"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "살필 폴더(opt, 기본 작업 폴더)"},
                "recursive": {"type": "boolean", "description": "하위 폴더까지 포함(opt)"},
                "hidden": {"type": "boolean", "description": "숨김 자리 포함(opt)"}
            },
            "required": []
        })
    }

    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            let args = match parse_tool_args(args_json) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            let path = match opt_str(&args, "path") {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let recursive = match bool_arg(&args, "recursive", false) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let hidden = match bool_arg(&args, "hidden", false) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let start = match resolve_dir(&ctx.working_dir, path) {
                Ok(dir) => dir,
                Err(reason) => return ToolOutcome::error(reason),
            };
            if !start.is_dir() {
                return ToolOutcome::error("폴더가 아닙니다");
            }
            let entries = collect(&ctx.working_dir, &start, recursive, hidden);
            let mut lines = Vec::with_capacity(entries.len());
            for entry in entries {
                lines.push(entry.rel);
            }
            ToolOutcome::Done(cap_lines(lines).join("\n"))
        })
    }
}
