//! 도구 호출 기록의 필드를 조립해 채우는 자리
//!
//! 호출 처리와 승인 흐름은 `executor.rs`의 책임이고, 이 파일은 기록 한 줄을 같은 필드로 채우는 일만 담당

use crate::agent::fallback::tool_call_form::ToolCallFormat;
use crate::agent::schema::types::ToolCallRecord;
use crate::llm::ToolCall;
use crate::tools::tool::{ToolContext, ToolOutcome};

use super::truncate::truncate_tool_result;

/// 호출 한 개의 실행 결과(기록과 승인 거부 표식)
pub(in crate::agent) struct RunOne {
    pub(in crate::agent) record: ToolCallRecord,
    pub(in crate::agent) needs_approval: bool,
}

impl RunOne {
    /// 승인표식 없는 plain 기록으로 상태 채움
    pub(in crate::agent) const fn plain(record: ToolCallRecord) -> Self {
        Self {
            record,
            needs_approval: false,
        }
    }

    /// 승인자 호출이 필요했던 기록임을 밝히는 상태
    pub(in crate::agent) const fn rejected(record: ToolCallRecord) -> Self {
        Self {
            record,
            needs_approval: true,
        }
    }
}

/// 도구 결과를 잘라 모델에 넘길 기록으로 채움
pub(in crate::agent) fn recorded(
    call: &ToolCall,
    outcome: &ToolOutcome,
    context: &ToolContext,
    executed_at: u64,
) -> ToolCallRecord {
    let text = truncate_tool_result(outcome.text(), context.tool_result_max_chars);
    record(call, text, executed_at)
}

/// 기록 항목을 채우는 한 곳(같은 필드 조립을 다시 적지 않음)
pub(in crate::agent) fn record(
    call: &ToolCall,
    result: String,
    executed_at: u64,
) -> ToolCallRecord {
    ToolCallRecord {
        call_id: call.id.clone(),
        tool_name: call.name.clone(),
        arguments: call.arguments.clone(),
        result,
        executed_at_unix_secs: executed_at,
        // 1안으로만 보내므로 기록의 호출 형식은 항상 1안
        format: ToolCallFormat::OpenAi,
    }
}
