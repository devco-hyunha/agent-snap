//! `execute` 도구: 코드 한 덩어리를 받아 03-8 브리지로 굴리고 요약 1개만 돌려줍니다(03-8 T2, spec AC 1·AC 4).
//!
//! 결론: 이 파일은 도구 등록 골격(이름·설명·스키마)과 승인 대상 표식만 두고, JS 엔진 구동과
//! 승인 게이트·기록은 `tools/execute_bridge/`가 맡습니다(dev-principles 2절 SRP).
//! 호출 전체 확인 1승인은 03-2 `executor`가 이 도구를 1안 경로에서 부를 때 채워 넘깁니다.
//! 스크립트 원문은 결과에 넣지 않고 요약만 올립니다(AC 4·AC 6).

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

/// `execute` 도구입니다.
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

    /// 코드 안의 쓰기·명령이 승인 게이트를 타므로, 이 도구는 승인 대상 도구로 등록합니다(D22 5.3절·AC 5).
    fn executes_writes(&self) -> bool {
        true
    }

    /// 인자 JSON의 `script` 본문(글자)을 받아 엔진에 넘기고, 결과 요약 1개와 호출 기록을
    /// 합친 문자열을 돌려줍니다(AC 1·AC 3·AC 4).
    ///
    /// 호출 전체 확인 1승인은 03-2 `executor`가 채워 넘깁니다. 승인 표식이 없는 채로
    /// 오면 실행하지 않고 승인 요청만 돌려줍니다(`run_command`과 같은 순서, AC 5의 앞승인 1개).
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

/// 결과 요약에 호출 기록 줄을 붙이는 자리입니다(AC 3: 기록을 로그에 남긴다).
fn append_log(summary: &str, log: &ScriptLog) -> String {
    if log.is_empty() {
        return summary.to_owned();
    }
    format!("{summary}\n기록: {}", log.text())
}
