//! `read_file` 도구: 텍스트 읽기 요청을 받아 경로 검사만 하고, 읽기 규칙은 `read_text`에 넘깁니다(03-3 T1, spec AC 1·1a).
//!
//! 경로 검사는 `path_guard`의 한 자리만 부르고, 잘림 표식과 4,000자 상한 정본은
//! 03-2의 `agent/record/truncate.rs`·`agent/settings/config.rs`를 그대로 재사용합니다(DRY).
//! 도구 스스로는 결과를 자르지 않습니다(03-1 결과 규격: 자르는 자리는 루프가 정본).
//! 크기 기준 3층(256KB·앞 128KB·50KiB 구간)은 D49에서 정한 값이며 `read_text`에 있습니다.

use serde_json::{Value, json};

use super::args::parse_tool_args;
use super::path_guard::check_tool_path;
use super::read_text::{FIRST_LINE, read_text};
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

/// `read_file` 도구입니다.
pub struct ReadFileTool;

impl ReadFileTool {
    const NAME: &'static str = "read_file";
}

impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        "텍스트 파일 읽기. 기본은 앞 128KB만 읽고 잘림을 표시한다. 256KB 이상은 읽지 않고, 긴 파일은 offset과 limit으로 구간을 읽는다"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "읽을 파일 경로"},
                "offset": {"type": "integer", "description": "구간 읽기 시작 줄(1부터)"},
                "limit": {"type": "integer", "description": "읽을 줄 수. 구간 한 장은 50KiB 이하"}
            },
            "required": ["path"]
        })
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
            // 경로 검사는 이 자리 한 곳에서만 부릅니다(spec AC 2·AC 4).
            let target = match check_tool_path(&ctx.working_dir, path) {
                Ok(target) => target,
                Err(guard) => return ToolOutcome::error(guard.message()),
            };
            let offset = match read_line_arg(&args, "offset") {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            let limit = match read_line_arg(&args, "limit") {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            read_text(&target, offset, limit)
        })
    }
}

/// 1부터 받는 정수 인자를 읽습니다. 없으면 `None`, 0 이하는 거부입니다.
fn read_line_arg(args: &Value, key: &str) -> Result<Option<usize>, &'static str> {
    let Some(raw) = args.get(key) else {
        return Ok(None);
    };
    if raw.is_null() {
        return Ok(None);
    }
    let number = raw
        .as_i64()
        .and_then(|value| usize::try_from(value).ok())
        .filter(|value| *value >= FIRST_LINE)
        .ok_or("offset과 limit은 1부터 정수여야 합니다")?;
    Ok(Some(number))
}
