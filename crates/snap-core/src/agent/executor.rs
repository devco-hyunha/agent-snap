//! 도구 호출 실행: `tool_calls`를 받아 결과를 만들고 결과를 잘라 모델에 넘길 한 줄로 바꿉니다.
//!
//! 이름 확인은 등록 목록(`registry.rs`)을, 인자 JSON 해석은 `args.rs`를, 잘림은
//! `truncate.rs`를 재사용합니다(SRP·DRY). 03-2 골격은 승인자를 두지 않으므로 승인 요청은
//! 즉시 거부로 보고하고, 상한·거부 판정(종료 4갈래)은 `runner.rs`의 판단에 넘깁니다.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::llm::ToolCall;
use crate::tools::tool::{ToolContext, ToolOutcome};

use super::args::recover_arguments;
use super::registry::ToolRegistry;
use super::truncate::truncate_tool_result;
use super::types::ToolCallRecord;

/// 도구 실행 한 바퀴가 만든 기록 목록입니다.
pub struct ToolRun {
    /// 모델에 넘긴 기록(결과 문자열은 잘림과 오류 한 줄을 함께 담습니다).
    pub records: Vec<ToolCallRecord>,
    /// 승인이 필요해서 거부한 호출 기록입니다. 03-2 골격은 승인자가 없어 항상 거부로 봅니다.
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

/// 모델이 부른 도구 호출 목록을 순서대로 실행해 기록으로 만듭니다(spec AC 1·AC 3).
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

/// 호출 한 개를 실행해 기록과 승인 거부 표식으로 바꿉니다.
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
    let outcome = match recover_arguments(&call.arguments) {
        Ok(args_json) => tool.call(&args_json, context).await,
        Err(reason) => ToolOutcome::error(&reason),
    };
    // 03-6에서 승인자가 채워질 때까지는 승인을 즉시 거부로 봅니다(D22 L1 기본).
    if matches!(outcome, ToolOutcome::ApprovalRequired { .. }) {
        let summary = outcome.text().to_owned();
        return RunOne::rejected(record(
            call,
            format!("오류: 승인 거부 {summary}"),
            executed_at,
        ));
    }
    let text = truncate_tool_result(outcome.text(), context.tool_result_max_chars);
    RunOne::plain(record(call, text, executed_at))
}

/// 호출 한 개의 실행 결과(기록과 승인 거부 표식)입니다.
struct RunOne {
    record: ToolCallRecord,
    needs_approval: bool,
}

impl RunOne {
    const fn plain(record: ToolCallRecord) -> Self {
        Self {
            record,
            needs_approval: false,
        }
    }

    const fn rejected(record: ToolCallRecord) -> Self {
        Self {
            record,
            needs_approval: true,
        }
    }
}

/// 기록 항목을 채우는 한 곳입니다(같은 필드 조립을 다시 적지 않습니다, DRY).
fn record(call: &ToolCall, result: String, executed_at: u64) -> ToolCallRecord {
    ToolCallRecord {
        call_id: call.id.clone(),
        tool_name: call.name.clone(),
        arguments: call.arguments.clone(),
        result,
        executed_at_unix_secs: executed_at,
    }
}

/// 지금 시각을 유닉스 초로 바꿉니다. 시험은 하한·상한으로만 비교합니다.
fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}
