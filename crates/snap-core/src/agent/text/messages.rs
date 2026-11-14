//! 2안 텍스트 프로토콜 messages 줄을 만드는 자리입니다(03-4 spec AC 5, D50 결정 5).
//!
//! 2안 왕복의 `assistant` 줄(`tool_calls` 없이 내용만)과 도구 결과를 `role: user` 글자로 messages에
//! 붙입니다(llama.cpp·mlx-lm 관례: `tool` 역할과 레거시 `name` 칸은 400을 부른다).
//! 1안 `tool` 줄 조립은 `prompt/assemble.rs`의 책임이고, 이 파일은 2안 전용만 만듭니다(관심사 분리).
//! 결과 글자를 잘림 상한까지 채워 조합하는 일까지 이 파일의 책임입니다(dev-principles 2절 SRP).

use crate::llm::ChatMessage;

use crate::agent::truncate::truncate_tool_result;

/// 2안 결과 전달 규격의 접두어입니다. 모델이 읽는 글자 형식입니다(spec AC 5).
pub const TEXT_RESULT_PREFIX: &str = "도구 결과";

/// 2안 assistant 내용 줄입니다. `tool_calls`를 넣지 않고 내용만 담습니다(D50 결정 5).
#[must_use]
pub fn text_assistant_message(content: &str) -> ChatMessage {
    ChatMessage {
        role: "assistant".into(),
        content: content.to_owned(),
        ..Default::default()
    }
}

/// 도구 결과를 `role: user`의 `도구 결과 [name]` 줄로 만듭니다(spec AC 5, D50 결정 5).
///
/// 잘림 상한은 `ToolContext`의 값(03-2 정본 4,000자)을 그대로 씁니다. 1안의 `role: tool` 줄과 달리
/// id를 글자에 넣지 않습니다(9자리 id 재투입 함정 방지, 12번 8절).
#[must_use]
pub fn text_result_message(name: &str, result: &str, max_chars: usize) -> ChatMessage {
    ChatMessage {
        role: "user".into(),
        content: format!(
            "{TEXT_RESULT_PREFIX} [{name}]\n{}",
            truncate_tool_result(result, max_chars)
        ),
        ..Default::default()
    }
}
