//! messages 조립 한 자리
//!
//! 왕복 도중에 생기는 `assistant`·`tool` 줄도 여기서 만듦

use crate::llm::{AssistantToolCall, ChatMessage, ToolCall};

use crate::agent::schema::types::ToolCallRecord;

/// 프롬프트 앞머리와 사용자 과제를 messages 순서대로 만듦
///
/// 빈 문자열 `agents_md`면 지시 없이 앞머리만 사용
#[must_use]
pub fn assemble_messages(
    system_prompt: &str,
    agents_md: &str,
    user_message: &str,
) -> Vec<ChatMessage> {
    let mut system_content = system_prompt.to_owned();
    if !agents_md.is_empty() {
        system_content.push('\n');
        system_content.push_str(agents_md);
    }
    vec![
        ChatMessage {
            role: "system".into(),
            content: system_content,
            ..Default::default()
        },
        ChatMessage {
            role: "user".into(),
            content: user_message.to_owned(),
            ..Default::default()
        },
    ]
}

/// 왕복 한 바퀴의 `assistant` messages 줄. 내용과 `tool_calls`를 한 자리에 담음
#[must_use]
pub fn assistant_turn_message(content: &str, calls: &[ToolCall]) -> ChatMessage {
    ChatMessage {
        role: "assistant".into(),
        content: content.to_owned(),
        tool_calls: calls
            .iter()
            .map(|call| AssistantToolCall {
                id: call.id.clone(),
                name: call.name.clone(),
                arguments: call.arguments.clone(),
            })
            .collect(),
        tool_call_id: None,
    }
}

/// 도구 결과를 `role:"tool"`·`tool_call_id`가 붙은 messages 줄로 변환
#[must_use]
pub fn tool_result_messages(records: &[ToolCallRecord]) -> Vec<ChatMessage> {
    records
        .iter()
        .map(|record| ChatMessage {
            role: "tool".into(),
            content: record.result.clone(),
            tool_calls: Vec::new(),
            tool_call_id: Some(record.call_id.clone()),
        })
        .collect()
}
