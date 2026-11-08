//! OpenAI가 받는 스키마 모양으로 메시지·도구 호출을 JSON으로 바꾸는 자리입니다.
//!
//! 본문 조립(`llm/request_body.rs`)은 필드를 채우는 책임이고, 서버 스키마가 바뀌는 코드는
//! 이 파일이 맡습니다(서로 다른 변경 이유 분리, D15).

use serde_json::{Map, Value};

use crate::llm::client::{AssistantToolCall, ChatMessage};

/// 메시지 한 줄을 OpenAI 스키마에 맞춰 JSON으로 바꿉니다.
///
/// `assistant`의 `tool_calls`와 `tool`의 `tool_call_id`는 값이 있을 때만 넣습니다.
/// 없는 필드는 본문에 넣지 않아 서버가 받지 않는 필드를 피합니다.
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

/// assistant가 보낸 도구 호출 목록을 OpenAI `tool_calls` 스키마로 바꿉니다.
///
/// 각 항목은 `id`·`type: function`·`function.name`·`function.arguments` 모양입니다.
/// 인자 JSON은 누적된 문자열을 그대로 넘깁니다(복구 판단은 03-2의 몫).
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
