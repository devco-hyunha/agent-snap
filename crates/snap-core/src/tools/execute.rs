//! `execute` 도구: 코드 한 덩어리를 받아 브리지로 굴리고 요약 1개만 반환.
//!
//! 이 파일은 도구 등록 골격(이름·설명·스키마)과 승인 대상 표식만 맡고, 엔진 구동·승인 게이트·기록은 `tools/execute_bridge/`가 맡음.

use serde_json::Value;
use serde_json::json;

use super::args::parse_tool_args;
use super::args::str_arg;
use super::execute_bridge::engine_drive::run_script;
use super::execute_bridge::engine_types::EngineBridge;
use super::execute_bridge::script_log::ScriptLog;
use super::execute_bridge::text_parts::script_approval_summary;
use super::tool::Tool;
use super::tool::ToolCallFuture;
use super::tool::ToolContext;
use super::tool::ToolOutcome;

/// `execute` 도구.
pub struct ExecuteTool;

impl Tool for ExecuteTool {
    fn name(&self) -> &str {
        "execute"
    }

    fn description(&self) -> &str {
        "여러 도구 호출을 코드로 조립해 한 번에 실행한다"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "script": {"type": "string", "description": "실행할 JS 코드 본문"}
            },
            "required": ["script"]
        })
    }

    /// 코드 안의 쓰기·명령이 승인 게이트를 타므로 승인 대상 도구로 등록.
    fn executes_writes(&self) -> bool {
        true
    }

    /// 인자 JSON의 `script` 본문을 엔진에 넘기고, 결과 요약과 호출 기록을 합친 문자열을 반환.
    ///
    /// 호출 전체 확인 승인은 `executor`가 채워 넘기며, 승인 표식이 없으면 실행하지 않고 승인 요청만 반환(`run_command`과 같은 순서).
    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            let args = match parse_tool_args(args_json) {
                Ok(value) => value,
                Err(reason) => return ToolOutcome::error(reason),
            };
            let Ok(script) = str_arg(&args, "script") else {
                return ToolOutcome::error("script는 문자열이어야 합니다");
            };
            if !ctx.take_approved() {
                return ToolOutcome::ApprovalRequired {
                    summary: script_approval_summary(script),
                };
            }
            let log = std::sync::Arc::new(ScriptLog::default());
            let bridge = EngineBridge::new(std::sync::Arc::new(ctx.clone()), log.clone());
            let summary = run_script(bridge, script);
            ToolOutcome::Done(append_log(&summary, &log))
        })
    }
}

/// 결과 요약에 호출 기록 줄을 붙임.
fn append_log(summary: &str, log: &ScriptLog) -> String {
    if log.is_empty() {
        return summary.to_owned();
    }
    format!("{summary}\n기록: {}", log.text())
}
