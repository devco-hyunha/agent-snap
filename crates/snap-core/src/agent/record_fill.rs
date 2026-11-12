//! 도구 호출 기록을 채우는 자리입니다(필드 조립).
//!
//! 호출 처리와 승인 흐름은 `executor.rs`의 책임이고, 이 파일은 기록 한 줄을 같은 필드로
//! 채우는 일만 맡습니다(03-6 리뷰 SRP 지적, dev-principles 2절). 시각 계산은 크레이트 공용
//! `time.rs`로 옮겼고(03-6 R1 감사 분리), 1안·2안 기록이 같은 시각을 함께 씁니다(DRY).

use crate::llm::ToolCall;
use crate::tools::tool::{ToolContext, ToolOutcome};

use super::tool_call_form::ToolCallFormat;
use super::truncate::truncate_tool_result;
use super::types::ToolCallRecord;

/// 호출 한 개의 실행 결과(기록과 승인 거부 표식)입니다.
pub(super) struct RunOne {
    pub(super) record: ToolCallRecord,
    pub(super) needs_approval: bool,
}

impl RunOne {
    /// 승인표식 없는 plain 기록으로 상태를 채웁니다.
    pub(super) const fn plain(record: ToolCallRecord) -> Self {
        Self {
            record,
            needs_approval: false,
        }
    }

    /// 승인자 호출이 필요했던 기록임을 밝히는 상태입니다.
    pub(super) const fn rejected(record: ToolCallRecord) -> Self {
        Self {
            record,
            needs_approval: true,
        }
    }
}

/// 도구 결과를 잘라 모델에 넘길 기록으로 채웁니다.
pub(super) fn recorded(
    call: &ToolCall,
    outcome: &ToolOutcome,
    context: &ToolContext,
    executed_at: u64,
) -> ToolCallRecord {
    let text = truncate_tool_result(outcome.text(), context.tool_result_max_chars);
    record(call, text, executed_at)
}

/// 기록 항목을 채우는 한 곳입니다(같은 필드 조립을 다시 적지 않습니다, DRY).
pub(super) fn record(call: &ToolCall, result: String, executed_at: u64) -> ToolCallRecord {
    ToolCallRecord {
        call_id: call.id.clone(),
        tool_name: call.name.clone(),
        arguments: call.arguments.clone(),
        result,
        executed_at_unix_secs: executed_at,
        // 03-2 루프는 1안으로만 보내므로 기록의 호출 형식은 항상 1안입니다(2안 기록은 03-4가 채움).
        format: ToolCallFormat::OpenAi,
    }
}
