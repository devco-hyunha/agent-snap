//! 왕복 스트림 끝까지 읽기와 결과 채움: 스트림을 끝까지 읽고 합치고, 왕복 한 바퀴 결과를 채움

use crate::llm::{AssembledResponse, ChatStream};

use crate::agent::fallback::switch::FallbackPoint;
use crate::agent::schema::types::{LoopResult, ToolCallRecord};

/// 스트림을 끝까지 읽고 합친 응답을 받아옴
pub(super) async fn drain_stream(mut stream: ChatStream) -> AssembledResponse {
    while let Some(event) = stream.next_event().await {
        if event.is_err() {
            break;
        }
    }
    stream.assembled().clone()
}

/// 왕복 한 바퀴의 결과를 채우는 한 곳(같은 조립을 다시 적지 않음)
///
/// 폴백 판단 값은 전환 기록(`switch_point`)을 그대로 사용
pub(super) fn loop_result(
    final_text: String,
    tool_calls: Vec<ToolCallRecord>,
    turns: u32,
    fallback_point: Option<FallbackPoint>,
) -> LoopResult {
    LoopResult {
        final_text,
        tool_calls,
        turns,
        fallback_point,
    }
}
