//! 1안 왕복 진행 상태: 왕복마다 바뀌는 값과 2안 전환 조립 한 자리를 맡습니다.

use crate::config::ToolCallForm;
use crate::llm::ChatRequest;

use super::fallback::FallbackPoint;
use super::registry::ToolRegistry;
use super::text::protocol::apply_text_assembly;
use super::tool_call_form::{ToolCallFormat, start_format};
use super::types::ToolCallRecord;

/// 1안 왕복의 진행 상태를 묶는 자리입니다(함수 인자 10개 넘김을 피하려는 구조, clippy `too_many_arguments` 대응).
///
/// 왕복마다 바뀌는 값만 한 곳으로 모습니다. 요청(messages를 키우는 자리)·왕복 형식·
/// 2안 전환 기록·도구 호출 기록이 그것입니다.
pub(super) struct LoopState {
    pub(super) request: ChatRequest,
    pub(super) format: ToolCallFormat,
    pub(super) switch_point: Option<FallbackPoint>,
    pub(super) records: Vec<ToolCallRecord>,
}

impl LoopState {
    /// 요청 골격과 선언(`tool_call_form`)에서 시작하는 상태를 만듭니다.
    pub(super) fn start(request: ChatRequest, declared: ToolCallForm) -> Self {
        Self {
            request,
            format: start_format(declared),
            switch_point: None,
            records: Vec::new(),
        }
    }

    /// 1안 왕복에서 2안 조립으로 전환합니다(4x 전환과 텍스트 2회 연속이 같은 조립, D50 결정 1).
    pub(super) fn switch_to_text(&mut self, registry: &ToolRegistry, point: FallbackPoint) {
        apply_text_assembly(&mut self.request, registry);
        self.format = ToolCallFormat::Text;
        self.switch_point = Some(point);
    }
}
