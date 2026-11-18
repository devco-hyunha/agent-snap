//! 2안 텍스트 프로토콜 messages 줄을 만드는 자리
//!
//! 1안 `tool` 줄 조립은 `prompt/assemble.rs`의 책임이고, 이 파일은 2안 전용만 만듦

use crate::llm::ChatMessage;

use crate::agent::truncate::truncate_tool_result;

/// 2안 결과 전달 규격의 접두어. 모델이 읽는 글자 형식
pub const TEXT_RESULT_PREFIX: &str = "도구 결과";

/// 2안 assistant 내용 줄. `tool_calls`를 넣지 않고 내용만 담음
#[must_use]
pub fn text_assistant_message(content: &str) -> ChatMessage {
    ChatMessage {
        role: "assistant".into(),
        content: content.to_owned(),
        ..Default::default()
    }
}

/// 도구 결과를 `role: user`의 `도구 결과 [name]` 줄로 생성
///
/// 1안의 `role: tool` 줄과 달리 id를 글자에 넣지 않음(id 재투입 함정 방지)
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
