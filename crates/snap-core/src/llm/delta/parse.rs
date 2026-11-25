//! 청크 JSON 해석 자리. `serde_json`으로 payload를 읽고 델타 필드를 뽑음.
//! JSONC 파서(`jsonc.rs`)가 아니라 표준 JSON 파서 사용. SSE payload는 주석 없는 JSON.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::llm::delta::chunk_error::ChunkError;
use crate::llm::delta::chunk_types::ChunkDelta;
use crate::llm::delta::chunk_types::FieldText;
use crate::llm::delta::chunk_types::ToolCallDelta;
use crate::llm::delta::types::Usage;

/// payload JSON 문자열을 해석해 델타 필드로 옮김.
pub fn parse_chunk(payload: &str) -> Result<ChunkDelta, ChunkError> {
    let value: Value =
        serde_json::from_str(payload).map_err(|e| ChunkError::from_json_error(&e))?;
    Ok(chunk_delta_of(&value))
}

/// `delta` 객체와 최상위 `usage`를 함께 뽑은 필드 묶음.
pub fn chunk_delta_of(value: &Value) -> ChunkDelta {
    let usage = usage_of(value);
    let Some(delta) = delta_object(value) else {
        return ChunkDelta {
            content: FieldText::Missing,
            reasoning: Vec::new(),
            tool_calls: Vec::new(),
            unknown_fields: BTreeMap::new(),
            usage,
        };
    };
    let content = field_text(delta, "content");
    let reasoning = vec![
        field_text(delta, "reasoning_content"),
        field_text(delta, "reasoning"),
    ];
    let tool_calls = tool_call_deltas(delta);
    let unknown_fields = unknown_fields(delta);
    ChunkDelta {
        content,
        reasoning,
        tool_calls,
        unknown_fields,
        usage,
    }
}

/// 최상위 객체의 `choices[0].delta` 객체.
fn delta_object(value: &Value) -> Option<&serde_json::Map<String, Value>> {
    let choice = value.get("choices")?.as_array()?.first()?;
    choice.get("delta")?.as_object()
}

/// 최상위 `usage` 객체를 사용량으로 옮김. 객체가 없으면 없음.
fn usage_of(value: &Value) -> Option<Usage> {
    let object = value.get("usage")?.as_object()?;
    Some(Usage {
        prompt_tokens: number(object, "prompt_tokens"),
        completion_tokens: number(object, "completion_tokens"),
        total_tokens: number(object, "total_tokens"),
    })
}

/// 숫자 필드 하나를 읽음. 숫자가 아니면 없음.
fn number(object: &serde_json::Map<String, Value>, key: &str) -> Option<u64> {
    object.get(key)?.as_u64()
}

/// 필드 하나를 문자열 값 상태로 읽음.
fn field_text(object: &serde_json::Map<String, Value>, key: &str) -> FieldText {
    match object.get(key) {
        None => FieldText::Missing,
        Some(Value::String(text)) => FieldText::Text(text.clone()),
        Some(_) => FieldText::NotText,
    }
}

/// `delta.tool_calls` 배열을 델타 목록으로 옮김. index가 없는 항목은 버림.
fn tool_call_deltas(delta: &serde_json::Map<String, Value>) -> Vec<ToolCallDelta> {
    let Some(raw) = delta.get("tool_calls").and_then(Value::as_array) else {
        return Vec::new();
    };
    raw.iter().filter_map(tool_call_delta).collect()
}

/// 배열 항목 하나를 도구 호출 델타로 옮김.
fn tool_call_delta(item: &Value) -> Option<ToolCallDelta> {
    let object = item.as_object()?;
    let index = object.get("index").and_then(Value::as_u64)? as usize;
    let function = object.get("function").and_then(Value::as_object);
    Some(ToolCallDelta {
        index,
        id: field_text(object, "id"),
        name: function_text(function, "name"),
        arguments: function_text(function, "arguments"),
    })
}

/// `function` 객체의 하위 필드를 문자열 값 상태로 읽음.
fn function_text(function: Option<&serde_json::Map<String, Value>>, key: &str) -> FieldText {
    let Some(function) = function else {
        return FieldText::Missing;
    };
    field_text(function, key)
}

/// 아는 필드를 뺀 `delta`의 나머지 필드 원문 JSON.
fn unknown_fields(delta: &serde_json::Map<String, Value>) -> BTreeMap<String, Value> {
    const KNOWN_KEYS: [&str; 4] = ["content", "reasoning_content", "reasoning", "tool_calls"];
    delta
        .iter()
        .filter(|(key, _)| !KNOWN_KEYS.contains(&key.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}
