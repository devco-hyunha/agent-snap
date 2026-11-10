//! `grep` 도구의 골격과 호출 배선입니다(03-3 T2).
//!
//! 결과 형식 3종(content·files·count) 모두 최대 200줄, 넘으면 잘림 표식을 붙입니다.
//! 숨김 자리는 기본 제외(opt로 포함), 비밀 파일 자리는 늘 제외합니다.
//! 실제 찾기 두 갈래(ripgrep 연동·내장 검색)는 각각 `ripgrep`·`search` 파일에 있습니다.

use serde_json::{Value, json};

use super::args::{bool_arg, opt_str, parse_tool_args, str_arg};
use super::display::cap_lines;
use super::ripgrep::search_with_ripgrep;
use super::search::search_builtin;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};
use super::walk::resolve_dir;

/// `grep` 도구입니다.
pub struct GrepTool;

impl GrepTool {
    const NAME: &'static str = "grep";
}

impl Tool for GrepTool {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        "파일 본문에서 패턴 찾기. ripgrep이 있으면 ripgrep, 없으면 내장 부분 문자열 검색. 형식 content·files·count, 최대 200줄"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {"type": "string", "description": "찾을 패턴(ripgrep 경로에서는 정규식)"},
                "path": {"type": "string", "description": "살필 폴더(opt, 기본 작업 폴더)"},
                "case_sensitive": {"type": "boolean", "description": "대소문자 구분(opt, 기본 true)"},
                "ext": {"type": "string", "description": "확장자 필터(opt, 예: rs 또는 .rs)"},
                "format": {"type": "string", "description": "출력 형식: content·files·count(opt, 기본 content)"},
                "hidden": {"type": "boolean", "description": "숨김 자리 포함(opt)"}
            },
            "required": ["pattern"]
        })
    }

    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            let args = match parse_tool_args(args_json) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            let pattern = match str_arg(&args, "pattern") {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let path = match opt_str(&args, "path") {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let case_sensitive = match bool_arg(&args, "case_sensitive", true) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let hidden = match bool_arg(&args, "hidden", false) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let ext = match opt_str(&args, "ext") {
                Ok(None) => None,
                Ok(Some(raw)) => {
                    let raw = raw.trim().trim_start_matches('.');
                    if raw.is_empty() {
                        return ToolOutcome::error("ext는 비어 있을 수 없습니다");
                    }
                    Some(raw.to_owned())
                }
                Err(reason) => return ToolOutcome::error(&reason),
            };
            let format = match opt_str(&args, "format") {
                Ok(None) => "content",
                Ok(Some(raw)) => raw,
                Err(reason) => return ToolOutcome::error(&reason),
            };
            if !matches!(format, "content" | "files" | "count") {
                return ToolOutcome::error("format은 content·files·count 중 하나여야 합니다");
            }
            let start = match resolve_dir(&ctx.working_dir, path) {
                Ok(dir) => dir,
                Err(reason) => return ToolOutcome::error(reason),
            };
            if !start.is_dir() {
                return ToolOutcome::error("폴더가 아닙니다");
            }
            let lines = match search_with_ripgrep(
                &ctx.working_dir,
                &start,
                pattern,
                case_sensitive,
                ext.as_deref(),
                format,
                hidden,
            ) {
                Some(lines) => lines,
                None => search_builtin(
                    &ctx.working_dir,
                    &start,
                    pattern,
                    case_sensitive,
                    ext.as_deref(),
                    format,
                    hidden,
                ),
            };
            ToolOutcome::Done(cap_lines(lines).join("\n"))
        })
    }
}
