//! 1안 왕복 진행 상태: 왕복마다 바뀌는 값과 2안 전환 조립 한 자리 담당

use crate::agent::fallback::connector::FallbackPoint;
use crate::agent::fallback::tool_call_form::{ToolCallFormat, start_format};
use crate::agent::record::registry::ToolRegistry;
use crate::agent::schema::types::ToolCallRecord;
use crate::agent::text::protocol::apply_text_assembly;
use crate::config::ToolCallForm;
use crate::llm::ChatRequest;

/// 1안 왕복의 진행 상태를 묶는 자리(함수 인자 과다를 피하려는 구조)
///
/// 왕복마다 바뀌는 값만 한곳으로 모음
pub(super) struct LoopState {
    pub(super) request: ChatRequest,
    pub(super) format: ToolCallFormat,
    pub(super) switch_point: Option<FallbackPoint>,
    pub(super) records: Vec<ToolCallRecord>,
}

impl LoopState {
    /// 요청 골격과 선언(`tool_call_form`)에서 시작하는 상태 생성
    pub(super) fn start(request: ChatRequest, declared: ToolCallForm) -> Self {
        Self {
            request,
            format: start_format(declared),
            switch_point: None,
            records: Vec::new(),
        }
    }

    /// 1안 왕복에서 2안 조립으로 전환(4x 전환과 텍스트 2회 연속이 같은 조립)
    pub(super) fn switch_to_text(&mut self, registry: &ToolRegistry, point: FallbackPoint) {
        apply_text_assembly(&mut self.request, registry);
        self.format = ToolCallFormat::Text;
        self.switch_point = Some(point);
    }
}
