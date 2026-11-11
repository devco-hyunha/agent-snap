//! messages를 조립하는 한 자리입니다(spec AC 1·AC 6, 동작 규칙 5).
//!
//! 프롬프트 앞머리로 `system`·`user` 줄을 만들고, 왕복 도중에 생기는 `assistant`·`tool` 줄도
//! 여기서 만듭니다. 루프는 조립된 줄을 messages에 붙이기만 합니다(dev-principles 2절 SRP).
//! `agent/system_prompt.txt` 내용을 `system` 메시지 한 자리에 앞머리로 담고,
//! 그 뒤에 03-5(`agents_md`)가 읽은 AGENTS.md 지시를 이어 붙입니다(spec AC 1).

use crate::llm::{AssistantToolCall, ChatMessage, ToolCall};

use super::types::ToolCallRecord;

/// 프롬프트 앞머리와 사용자 과제를 messages 순서대로 만듭니다.
///
/// 순서는 `system`(프롬프트 앞머리) → `user`(사용자 과제)이며, 이 순서가 AC 6 테스트로 고정됩니다.
/// `agents_md`는 03-5(`agents_md`)가 읽은 AGENTS.md 지시 원문이고, 빈 문자열이면 지시 없이 앞머리만 씁니다(spec AC 2·AC 3).
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

/// 왕복 한 바퀴의 `assistant` messages 줄입니다: 내용과 `tool_calls`를 한 자리에 담습니다.
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

/// 도구 결과를 `role:"tool"`·`tool_call_id`가 붙은 messages 줄로 바꿉니다.
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
