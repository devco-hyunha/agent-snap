//! OpenAI 스키마 모양으로 메시지·도구 호출을 JSON으로 바꿈.
//!
//! 본문 조립은 필드 채움, 이 파일은 서버 스키마 변환만 맡음.

use serde_json::{Map, Value};

use crate::llm::client::{AssistantToolCall, ChatMessage};

/// 메시지 한 줄을 OpenAI 스키마에 맞춰 JSON으로 바꿈.
///
/// `tool_calls`·`tool_call_id`는 값이 있을 때만 넣어 서버가 받지 않는 필드를 피함.
pub(crate) fn message_json(message: &ChatMessage) -> Value {
    let mut item = Map::new();
    item.insert("role".into(), Value::String(message.role.clone()));
    item.insert("content".into(), Value::String(message.content.clone()));
    if !message.tool_calls.is_empty() {
        item.insert("tool_calls".into(), tool_calls_json(&message.tool_calls));
    }
    if let Some(id) = &message.tool_call_id {
        item.insert("tool_call_id".into(), Value::String(id.clone()));
    }
    Value::Object(item)
}

/// assistant 도구 호출 목록을 OpenAI `tool_calls` 스키마로 바꿈.
///
/// 인자 JSON은 누적 문자열을 그대로 넘김.
pub(crate) fn tool_calls_json(calls: &[AssistantToolCall]) -> Value {
    Value::Array(
        calls
            .iter()
            .map(|call| {
                let mut function = Map::new();
                function.insert("name".into(), Value::String(call.name.clone()));
                function.insert("arguments".into(), Value::String(call.arguments.clone()));
                let mut item = Map::new();
                item.insert("id".into(), Value::String(call.id.clone()));
                item.insert("type".into(), Value::String("function".into()));
                item.insert("function".into(), Value::Object(function));
                Value::Object(item)
            })
            .collect(),
    )
}
