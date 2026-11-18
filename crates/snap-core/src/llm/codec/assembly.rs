//! 델타 누적 창구를 한곳에 모은 조립 래퍼.
//!
//! 이벤트를 같은 규칙으로 합친 응답에 키움. `ChatStream`은 이 창구에 위임.

use crate::llm::codec::delta::{AssembledResponse, DeltaAssembler, StreamEvent};

/// 스트림 이벤트 누적 창구. 이벤트 하나를 합친 응답에 더함.
#[derive(Debug, Default)]
pub struct ChatAssembly {
    assembler: DeltaAssembler,
}

impl ChatAssembly {
    /// 빈 누적 창구를 만듦.
    pub fn new() -> Self {
        Self::default()
    }

    /// 이벤트 하나를 지금까지 합친 응답에 누적.
    pub fn apply_event(&mut self, event: &StreamEvent) {
        self.assembler.apply_event(event);
    }

    /// 지금까지 누적한 합친 응답의 참조를 반환.
    pub fn assembled(&self) -> &AssembledResponse {
        self.assembler.assembled()
    }
}
