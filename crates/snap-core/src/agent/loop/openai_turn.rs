//! 1안 왕복 판단·이어쓰기: 호출 없는 텍스트 판단과 assistant·도구 결과 붙이기를 맡습니다.

use crate::llm::{ChatMessage, ToolCall};

use crate::agent::fallback::connector::{
    FallbackPoint, first_turn_fallback_point, is_text_switch_streak,
};
use crate::agent::prompt::assemble::assistant_turn_message;
use crate::agent::record::registry::ToolRegistry;
use crate::agent::text::messages::text_assistant_message;

use super::loop_state::LoopState;

/// 호출 없는 텍스트 왕복이 2안 전환인지, 1안 후보로 이어가는지, 선언대로 끝내는지를 밝힙니다.
pub(super) enum TextStep {
    /// 텍스트 2회 연속에서 2안 조립을 붙였습니다(전환 확정).
    Switch,
    /// 후보 1회이고 1안 assistant 줄을 이어쓰기에 붙였습니다.
    KeepCandidate,
    /// 선언 값(`openai`·`text`)은 감지를 쓰지 않으므로 AC 1대로 종료합니다.
    DeclaredStop,
}

/// 1안 왕복에서 도구 호출 없이 온 텍스트의 전환 판단입니다(D50 결정 2, spec AC 1).
///
/// 감지(`auto`)가 아닌 선언은 전환 기록 값만 채우고 AC 1대로 끝냅니다.
/// 2회 연속에서는 2안 조립을 붙이고 assistant 내용 줄만 이어쓰기에 붙입니다.
/// 후보 1회는 전환하지 않고 1안 assistant 줄(도구 호출 없음)만 붙이고, 2회 연속 계수를 더합니다.
/// 앞의 전환 기록(4x)이 있으면 그 기록을 지우지 않고 같은 자리에만 채웁니다.
pub(super) fn apply_text_only_turn(
    state: &mut LoopState,
    registry: &ToolRegistry,
    content: &str,
    calls: &[ToolCall],
    turns: u32,
    text_turns: &mut u32,
    detects: bool,
) -> TextStep {
    *text_turns += 1;
    let candidate_point = || first_turn_fallback_point(true, turns, !state.records.is_empty());
    if detects && is_text_switch_streak(*text_turns) {
        state.switch_to_text(registry, FallbackPoint::TextStreak);
        state.request.messages.push(text_assistant_message(content));
        return TextStep::Switch;
    }
    state.switch_point = state.switch_point.or_else(candidate_point);
    if !detects {
        return TextStep::DeclaredStop;
    }
    // 후보 1회는 전환하지 않고 1안으로 한 번 더 보냅니다(spec AC 1).
    // 후보 판단 값은 assistant 내용 줄을 붙여 이어쓰기에 남깁니다(D50 결정 5).
    state
        .request
        .messages
        .push(assistant_turn_message(content, calls));
    TextStep::KeepCandidate
}

/// 1안 왕복의 assistant·도구 결과 줄을 messages 끝에 붙이는 한 자리입니다(이어쓰기 조립, DRY).
pub(super) fn append_openai_roundtrip(
    state: &mut LoopState,
    content: &str,
    calls: &[ToolCall],
    tool_messages: Vec<ChatMessage>,
) {
    state
        .request
        .messages
        .push(assistant_turn_message(content, calls));
    state.request.messages.extend(tool_messages);
}
