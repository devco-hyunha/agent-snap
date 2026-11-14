//! 도구 호출을 처리해 모델에 넘길 기록으로 바꿉니다(승인 흐름 포함).
//!
//! 이름 확인은 등록 목록(`record/registry.rs`)을, 인자 JSON 해석은 `fallback/args.rs`를,
//! 잘림은 `record/truncate.rs`를 재사용합니다(SRP·DRY). 03-6이 승인 흐름의 창구로 이 파일을
//! 씁니다: 승인이 필요한 도구는 `ToolContext.approver`를 불러 승인·거부·만료를 정하고(D22),
//! 승인자 미주입은 즉시 거부, 만료는 거부로 기록합니다. 상한·거부 판정(종료 4갈래)은
//! `loop/runner.rs`의 판단에 넘깁니다. 기록 필드 조립과 시각은 `record/record_fill.rs`의 책임입니다
//! (03-6 리뷰 SRP 지적, dev-principles 2절).

use crate::agent::fallback::args::recover_arguments;
use crate::agent::record::record_fill::{RunOne, record, recorded};
use crate::agent::record::registry::ToolRegistry;
use crate::agent::schema::types::ToolCallRecord;
use crate::llm::ToolCall;
use crate::time::now_unix_secs;
use crate::tools::approver::request_approval;
use crate::tools::tool::{Tool, ToolContext, ToolOutcome};

/// 도구 호출 한 바퀴가 만든 기록 목록입니다.
pub struct ToolRun {
    /// 모델에 넘긴 기록(결과 문자열은 잘림과 오류 한 줄을 함께 담습니다).
    pub records: Vec<ToolCallRecord>,
    /// 승인이 필요해서 거부한 호출 기록입니다. 03-6 승인자가 채워지기 전에는 항상 거부로 봅니다.
    pub rejection: Option<ToolCallRecord>,
}

impl ToolRun {
    /// 기록 목록과 거부 기록을 한꺼번에 빼냅니다.
    ///
    /// 루프(`runner.rs`)가 기록은 이동하고 거부 기록은 값으로만 씁니다. 같은 값을 두고
    /// 빌림과 이동이 뒤섞이지 않게 하는 자리이고, 마지막 기록을 고르는 식으로 다시 얻지 않습니다.
    pub fn into_parts(self) -> (Vec<ToolCallRecord>, Option<ToolCallRecord>) {
        (self.records, self.rejection)
    }
}

/// 모델이 부른 도구 호출 목록을 순서대로 처리해 기록으로 만듭니다(spec AC 1·AC 3).
///
/// 등록에 없는 이름은 `도구 없음: 이름` 한 줄, 인자 JSON이 깨지면 복구 뒤에
/// `오류: 인자 JSON 형식 오류` 한 줄을 결과로 남깁니다.
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

/// 호출 한 개를 처리해 기록과 승인 거부 표식으로 바꿉니다.
///
/// 승인이 필요한 도구는 03-6 승인자(`CommandApprover`)를 불러 결과를 정합니다.
/// 승인자 미주입은 기다리지 않고 즉시 거부하고, 응답 상한 만료는 거부로만 봅니다(D22: L3 무인 아님).
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

/// 승인자 응답이 승인한 호출만 셸·도구에 넘기고, 표식이 다음 호출으로 새지 않게 닫습니다.
///
/// 표식을 이 호출 한 개에 채우고 부른 뒤 반드시 지웁니다(D22 5.3절·spec AC 1: 명령마다 확인).
/// 도구가 표식을 소비하지 않고 끝나도 다음 호출은 다시 승인자를 타야 합니다.
async fn run_approved_call(
    tool: &dyn Tool,
    args_json: &str,
    context: &ToolContext,
    summary: &str,
) -> ToolOutcome {
    context.mark_approved();
    let outcome = tool.call(args_json, context).await;
    context.clear_approved();
    if matches!(outcome, ToolOutcome::ApprovalRequired { .. }) {
        // 승인을 채워 불렀는데 다시 승인 요청이 나오면 실행하지 않고 거부로 기록합니다.
        return ToolOutcome::error(&format!("승인 거부(재승인 요청) {summary}"));
    }
    outcome
}
