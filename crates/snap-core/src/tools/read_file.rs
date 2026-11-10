//! `read_file` 도구: 텍스트 읽기·크기 상한·잘림 표시·구간 읽기(03-3 T1, spec AC 1·1a).
//!
//! 경로 검사는 `path_guard`의 한 자리만 부르고, 잘림 표식과 4,000자 상한 정본은
//! 03-2의 `agent/truncate.rs`·`agent/config.rs`를 그대로 재사용합니다(DRY).
//! 도구 스스로는 결과를 자르지 않습니다(03-1 결과 규격: 자르는 자리는 루프가 정본).
//! 크기 기준 3층(256KB·앞 128KB·50KiB 구간)은 D49에서 정한 값입니다.

use std::path::Path;

use serde_json::{Value, json};

use crate::agent::TRUNCATION_MARK;

use super::args::parse_tool_args;
use super::path_guard::check_tool_path;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

/// 기본 읽기를 거부하는 파일 크기입니다(D49 1층: 256KB 이상).
const REJECT_FILE_BYTES: u64 = 256 * 1024;
/// 기본 읽기가 읽는 앞부분 크기입니다(D49 2층: 앞 128KB만).
const BASE_READ_BYTES: usize = 128 * 1024;
/// 구간 읽기 한 장 상한입니다(D49 3층: 50KiB 이하).
const SECTION_PAGE_BYTES: usize = 50 * 1024;
/// 첫 줄 번호입니다. `offset`을 적지 않으면 이 줄부터 읽습니다.
const FIRST_LINE: usize = 1;
/// 기본 읽기를 거부할 때 남기는 안내입니다(spec AC 1a: `offset`·`limit`·grep).
const BASE_REJECT_HINT: &str =
    "읽기 한도 초과(256KB 이상). offset과 limit으로 구간을 읽거나 grep으로 검색하세요";

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

/// 파일을 읽어 도구 결과로 바꿉니다. `offset`·`limit`이 둘 다 없으면 기본 읽기입니다.
///
/// 기본 읽기에서 256KB 이상이면 읽지 않고 안내를 남깁니다. 구간을 지정하면 256KB 넘는
/// 파일도 읽을 수 있습니다(spec AC 1a).
fn read_text(target: &Path, offset: Option<usize>, limit: Option<usize>) -> ToolOutcome {
    let Ok(meta) = std::fs::metadata(target) else {
        return ToolOutcome::error("파일을 찾을 수 없습니다");
    };
    if !meta.is_file() {
        return ToolOutcome::error("파일이 없습니다");
    }
    let section = offset.is_some() || limit.is_some();
    if !section && meta.len() >= REJECT_FILE_BYTES {
        return ToolOutcome::error(BASE_REJECT_HINT);
    }
    let Ok(bytes) = std::fs::read(target) else {
        return ToolOutcome::error("파일을 읽지 못했습니다");
    };
    let Ok(text) = String::from_utf8(bytes) else {
        return ToolOutcome::error("바이너리 파일은 읽지 않습니다");
    };
    match (offset, limit) {
        (None, None) => ToolOutcome::Done(base_read(&text)),
        (offset, limit) => match section_read(&text, offset.unwrap_or(FIRST_LINE), limit) {
            Ok(section) => ToolOutcome::Done(section),
            Err(reason) => ToolOutcome::error(&reason),
        },
    }
}

/// 기본 읽기: 앞 128KB만 줄 번호와 함께 보여주고, 잘렸으면 표식을 붙입니다(spec AC 1).
fn base_read(text: &str) -> String {
    let head = head_bytes(text, BASE_READ_BYTES);
    let mut out = numbered_lines(head, FIRST_LINE);
    if head.len() < text.len() {
        out.push_str(TRUNCATION_MARK);
    }
    out
}

/// 구간 읽기: `offset` 줄부터 `limit` 줄까지를 줄 번호와 함께 보여줍니다(spec AC 1a).
///
/// 구간 한 장이 50KiB를 넘으면 읽지 않고 이유를 돌려줍니다(D49 3층).
fn section_read(text: &str, offset: usize, limit: Option<usize>) -> Result<String, String> {
    let selected: Vec<&str> = text
        .lines()
        .skip(offset.saturating_sub(FIRST_LINE))
        .take(limit.unwrap_or(usize::MAX))
        .collect();
    // 한 줄에 개행 한 글자를 더한 크기가 구간 한 장의 기준입니다.
    let page_bytes: usize = selected.iter().map(|line| line.len() + 1).sum();
    if page_bytes > SECTION_PAGE_BYTES {
        return Err(format!(
            "구간 한 장이 {}KiB를 넘습니다. limit을 줄이세요",
            SECTION_PAGE_BYTES / 1024
        ));
    }
    let joined = selected.join("\n");
    Ok(numbered_lines(&joined, offset))
}

/// 텍스트에서 앞 `max_bytes` 바이트까지를 글자 경계에서 잘라 줍니다.
///
/// UTF-8 글자 한가운데에서 자르면 결과가 깨지므로 글자 경계까지 내려갑니다.
fn head_bytes(text: &str, max_bytes: usize) -> &str {
    if text.len() <= max_bytes {
        return text;
    }
    let mut cut = max_bytes;
    while cut > 0 && !text.is_char_boundary(cut) {
        cut -= 1;
    }
    &text[..cut]
}

/// 줄마다 번호를 붙여 한 줄씩 이어 씁니다. 번호는 `start`부터 매깁니다.
///
/// 구간 읽기도 기본 읽기도 같은 표기를 쓰므로 이 한 곳에서만 만듭니다(DRY).
/// 도구 바깥에서 쓰지 않으므로 이 파일 안에서만 씁니다.
#[must_use]
fn numbered_lines(text: &str, start: usize) -> String {
    text.lines()
        .enumerate()
        .map(|(index, line)| format!("{}: {line}\n", start + index))
        .collect()
}
