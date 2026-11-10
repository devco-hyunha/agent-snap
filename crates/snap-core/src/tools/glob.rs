//! `glob` 도구: 이름 패턴에 맞는 자리를 찾습니다(03-3 T2).
//!
//! 패턴 글자: `*`는 이름 안 여러 글자, `?`는 한 글자, `**`는 폴더 사이까지 건넙니다.
//! 숨김 자리는 기본 제외(opt로 포함), 비밀 파일 자리는 늘 제외합니다.
//! 이름은 작업 폴더 기준 상대 경로로 보여 줍니다.

use serde_json::{Value, json};

use super::args::{bool_arg, opt_str, parse_tool_args, str_arg};
use super::display::cap_lines;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};
use super::walk::{collect, resolve_dir};

/// `glob` 도구입니다.
pub struct GlobTool;

impl GlobTool {
    const NAME: &'static str = "glob";
}

impl Tool for GlobTool {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        "이름 패턴에 맞는 경로 목록. `*`·`?`·`**` 지원, 숨김 기본 제외(opt 포함), 비밀 파일 제외"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {"type": "string", "description": "이름 패턴. `*`는 한 이름 안, `**`는 폴더 사이"},
                "path": {"type": "string", "description": "살필 폴더(opt, 기본 작업 폴더)"},
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
            let entries = collect(&ctx.working_dir, &start, true, hidden);
            let mut lines = Vec::new();
            for entry in entries {
                if match_pattern(pattern, &entry.rel) {
                    lines.push(entry.rel);
                }
            }
            ToolOutcome::Done(cap_lines(lines).join("\n"))
        })
    }
}

/// `*`·`?`·`**`만 지원하는 작은 이름 패턴 매처입니다.
///
/// 경로를 `/` 구분 상대 문자열로 보고, `*`는 구분자를 넘지 않습니다.
fn match_pattern(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let n = p.len();
    let m = t.len();
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[n][m] = true;
    for i in (0..n).rev() {
        for j in (0..=m).rev() {
            dp[i][j] = match p[i] {
                '*' => {
                    if i + 1 < n && p[i + 1] == '*' {
                        let after = if i + 2 < n && p[i + 2] == '/' {
                            i + 3
                        } else {
                            i + 2
                        };
                        dp[after][j] || (j < m && dp[i][j + 1])
                    } else {
                        let mut hit = dp[i + 1][j];
                        let mut k = j;
                        while !hit && k < m && t[k] != '/' {
                            k += 1;
                            hit = dp[i + 1][k];
                        }
                        hit
                    }
                }
                '?' => j < m && t[j] != '/' && dp[i + 1][j + 1],
                other => j < m && char_eq(other, t[j]) && dp[i + 1][j + 1],
            };
        }
    }
    dp[0][0]
}

/// 글자 비교. Windows는 대소문자를 가리지 않고, 그 밖의 OS는 가립니다.
fn char_eq(a: char, b: char) -> bool {
    if cfg!(windows) {
        a.eq_ignore_ascii_case(&b)
    } else {
        a == b
    }
}
