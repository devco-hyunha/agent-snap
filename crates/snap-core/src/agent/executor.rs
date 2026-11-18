//! 도구 호출을 처리해 모델에 넘길 기록으로 바꿈(승인 흐름 포함).
//!
//! 승인 필요 도구는 `ToolContext.approver` 창구로 감. 미주입은 즉시 거부, 만료는 거부로 기록.

use crate::agent::fallback::args::recover_arguments;
use crate::agent::record::record_fill::{RunOne, record, recorded};
use crate::agent::record::registry::ToolRegistry;
use crate::agent::schema::types::ToolCallRecord;
use crate::llm::ToolCall;
use crate::time::now_unix_secs;
use crate::tools::approver::request_approval;
use crate::tools::tool::{Tool, ToolContext, ToolOutcome};

/// 도구 호출 한 바퀴가 만든 기록 목록
pub struct ToolRun {
    /// 모델에 넘긴 기록. 결과 문자열은 잘림과 오류 한 줄 포함
    pub records: Vec<ToolCallRecord>,
    /// 승인이 필요해 거부한 호출 기록. 승인자 미채움 시 항상 거부로 봄
    pub rejection: Option<ToolCallRecord>,
}

impl ToolRun {
    /// 기록 목록과 거부 기록을 한꺼번에 댬
    ///
    /// 빌림과 이동이 뒤섞이지 않게 값으로만 얻는 자리
    pub fn into_parts(self) -> (Vec<ToolCallRecord>, Option<ToolCallRecord>) {
        (self.records, self.rejection)
    }
}

/// 모델이 부른 도구 호출 목록을 순서대로 처리해 기록으로 만듦
///
/// 등록에 없는 이름과 깨진 인자 JSON은 결과에 한 줄로 남김
#[must_use]
pub async fn run_tool_calls(
    registry: &ToolRegistry,
    context: &ToolContext,
    calls: &[ToolCall],
) -> ToolRun {
    let executed_at = now_unix_secs();
    let mut records = Vec::with_capacity(calls.len());
    let mut rejection: Option<ToolCallRecord> = None;
    for call in calls {
        let RunOne {
            record,
            needs_approval,
        } = run_one(registry, context, call, executed_at).await;
        if needs_approval && rejection.is_none() {
            rejection = Some(record.clone());
        }
        records.push(record);
    }
    ToolRun { records, rejection }
}

/// 호출 한 개를 처리해 기록과 승인 거부 표식으로 바꿈
///
/// 승인 필요 도구는 승인자(`CommandApprover`)로 결과를 정함
async fn run_one(
    registry: &ToolRegistry,
    context: &ToolContext,
    call: &ToolCall,
    executed_at: u64,
) -> RunOne {
    let Some(tool) = registry.find(&call.name) else {
        let text = format!("도구 없음: {}", call.name);
        return RunOne::plain(record(call, text, executed_at));
    };
    let args_json = match recover_arguments(&call.arguments) {
        Ok(args_json) => args_json,
        Err(reason) => {
            let text = ToolOutcome::error(&reason).text().to_owned();
            return RunOne::plain(record(call, text, executed_at));
        }
    };
    let outcome = tool.call(&args_json, context).await;
    if matches!(outcome, ToolOutcome::ApprovalRequired { .. }) {
        let summary = outcome.text().to_owned();
        let reply = request_approval(
            context.approver.as_ref(),
            &summary,
            &context.run_command,
            &context.cancel_token(),
        )
        .await;
        if reply.is_approved() {
            return RunOne::plain(recorded(
                call,
                &run_approved_call(tool, &args_json, context, &summary).await,
                context,
                executed_at,
            ));
        }
        return RunOne::rejected(record(call, reply.rejection_text(&summary), executed_at));
    }
    RunOne::plain(recorded(call, &outcome, context, executed_at))
}

/// 승인자 응답이 승인한 호출만 셸·도구에 넘기고, 표식이 다음 호출으로 새지 않게 닫음
///
/// 표식은 이 호출 한 개에만 채우고 부른 뒤 반드시 지웁
pub(crate) async fn run_approved_call(
    tool: &dyn Tool,
    args_json: &str,
    context: &ToolContext,
    summary: &str,
) -> ToolOutcome {
    context.mark_approved();
    let outcome = tool.call(args_json, context).await;
    context.clear_approved();
    if matches!(outcome, ToolOutcome::ApprovalRequired { .. }) {
        // 승인을 채워 불렀는데 다시 승인 요청이 나오면 실행하지 않고 거부로 기록
        return ToolOutcome::error(&format!("승인 거부(재승인 요청) {summary}"));
    }
    outcome
}
