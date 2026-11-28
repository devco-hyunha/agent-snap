//! 델타 누적 계산 자리. 청크에서 뽑은 필드를 누적 칸에 붙이고 교체하는 계산만 담음.
//!
//! 누적 상태 타입은 `state`가, 필드 타입은 `chunk_types`가, 해석은 `parse`가 맡음.

use crate::llm::delta::chunk_types::ChunkDelta;
use crate::llm::delta::chunk_types::FieldText;
use crate::llm::delta::chunk_types::ToolCallDelta;
use crate::llm::delta::types::Delta;
use crate::llm::delta::types::ToolCall;
use crate::llm::delta::types::Usage;

/// 해석된 델타 필드 묶음을 누적 응답에 합침.
pub(super) fn apply_chunk_fields(delta: &mut Delta, chunk: &ChunkDelta) {
    append_text(&mut delta.content, &chunk.content);
    for reasoning in &chunk.reasoning {
        append_text(&mut delta.reasoning, reasoning);
    }
    for tool_call in &chunk.tool_calls {
        apply_tool_call(&mut delta.tool_calls, tool_call);
    }
    for (key, value) in &chunk.unknown_fields {
        delta.unknown_fields.insert(key.clone(), value.clone());
    }
    apply_usage(&mut delta.usage, chunk.usage);
}

/// 문자열 값이 온 순서대로 누적 칸에 붙임.
fn append_text(target: &mut String, field: &FieldText) {
    let FieldText::Text(text) = field else {
        return;
    };
    target.push_str(text);
}

/// 사용량은 온 청크 값으로 교체. 마지막에 온 값이 남음.
fn apply_usage(target: &mut Option<Usage>, incoming: Option<Usage>) {
    let Some(usage) = incoming else {
        return;
    };
    *target = Some(usage);
}

/// 도구 호출 델타를 index별로 합침. 서로 다른 index가 번갈아 와도 index별로 붙음.
fn apply_tool_call(tool_calls: &mut Vec<ToolCall>, delta: &ToolCallDelta) {
    let Some(position) = tool_calls
        .iter()
        .position(|tool_call| tool_call.index == delta.index)
    else {
        // index가 처음이면 새 항목을 뒤에 추가함.
        tool_calls.push(new_tool_call(delta));
        return;
    };
    let tool_call = &mut tool_calls[position];
    keep_first_id(tool_call, &delta.id);
    append_text(&mut tool_call.name, &delta.name);
    append_text(&mut tool_call.arguments, &delta.arguments);
}

/// index별 도구 호출이 없으면 id·이름·인자를 함께 채워 추가함.
fn new_tool_call(delta: &ToolCallDelta) -> ToolCall {
    let id = match &delta.id {
        FieldText::Text(text) => Some(text.clone()),
        FieldText::NotText | FieldText::Missing => None,
    };
    let mut tool_call = ToolCall {
        index: delta.index,
        id,
        name: String::new(),
        arguments: String::new(),
    };
    append_text(&mut tool_call.name, &delta.name);
    append_text(&mut tool_call.arguments, &delta.arguments);
    tool_call
}

/// id는 첫 번째로 온 값만 남김.
fn keep_first_id(tool_call: &mut ToolCall, field: &FieldText) {
    let FieldText::Text(text) = field else {
        return;
    };
    if tool_call.id.is_some() {
        return;
    }
    tool_call.id = Some(text.clone());
}
