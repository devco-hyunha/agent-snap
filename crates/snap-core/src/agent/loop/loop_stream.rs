//! 왕복 스트림 드레인과 결과 채움: 스트림을 끝까지 읽고 합치고, 왕복 한 바퀴 결과를 채웁니다.

use crate::llm::{AssembledResponse, ChatStream};

use crate::agent::fallback::connector::FallbackPoint;
use crate::agent::schema::types::{LoopResult, ToolCallRecord};

/// 스트림을 끝까지 읽고 합친 응답을 받아옵니다.
pub(super) async fn drain_stream(mut stream: ChatStream) -> AssembledResponse {
    while let Some(event) = stream.next_event().await {
        if event.is_err() {
            break;
        }
    }
    stream.assembled().clone()
}

/// 왕복 한 바퀴의 결과를 채우는 한 곳입니다(같은 조립을 다시 적지 않습니다, DRY).
///
/// 폴백 판단 값은 첫 턴 판정 경로와 03-4 전환 기록(`switch_point`)에서 채우고, 그 밖의 종료
/// 자리(반복 상한·거부)는 `None`입니다.
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
