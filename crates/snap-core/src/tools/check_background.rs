//! `check_background` 도구: 백그라운드로 옮긴 명령을 wake 주기마다 한 번 확인해 결과를 넘김
//!
//! wake 한 번은 에이전트 턴 하나와 대응하고, 목록에 명령이 없으면 확인하지 않은 상태로 한 줄만 남김

use serde_json::json;

use super::background::shared::lock_background;
use super::tool::{Tool, ToolCallFuture, ToolContext, ToolOutcome};

/// `check_background` 도구
pub struct CheckBackgroundTool;

impl Tool for CheckBackgroundTool {
    fn name(&self) -> &str {
        "check_background"
    }

    fn description(&self) -> &str {
        "백그라운드로 옮긴 명령을 순서대로 한 번 확인한다. 확인할 명령이 없으면 즉시 알린다"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            let Ok(_args) = super::args::parse_tool_args(args_json) else {
                return ToolOutcome::error("인자를 읽지 못했습니다");
            };
            let Some(mut command) = lock_background(&ctx.background).take_for_wake() else {
                return ToolOutcome::Done("백그라운드 명령이 없습니다".to_owned());
            };
            let outcome = command.wake(ctx.wake_interval).await;
            if command.is_finished() {
                let _ = command.release().await;
            } else {
                lock_background(&ctx.background).reinsert(command);
            }
            ToolOutcome::Done(outcome)
        })
    }
}
