//! 델타 누적 창구를 한곳에 모은 조립 래퍼입니다.
//!
//! `DeltaAssembler`가 만든 이벤트를 같은 누적 규칙으로 합친 응답으로 키우는 자리만 맡습니다(합친 응답 재사용, DRY).
//! 스트림 손잡이(`ChatStream`)는 이 창구를 필드로 위임하고, 누적 규칙은 이 파일이 정본으로 지킵니다(SRP).

use crate::llm::codec::delta::{AssembledResponse, DeltaAssembler, StreamEvent};

/// 스트림 이벤트 누적 창구입니다. 이벤트 하나를 합친 응답에 더하고, 지금까지의 결과를 보여줍니다.
#[derive(Debug, Default)]
pub struct ChatAssembly {
    assembler: DeltaAssembler,
}

impl ChatAssembly {
    /// 빈 누적 창구를 만듭니다.
    pub fn new() -> Self {
        Self::default()
    }

    /// 이벤트 하나를 지금까지 합친 응답에 누적합니다.
    pub fn apply_event(&mut self, event: &StreamEvent) {
        self.assembler.apply_event(event);
    }

    /// 지금까지 누적한 합친 응답의 참조를 돌려줍니다.
    pub fn assembled(&self) -> &AssembledResponse {
        self.assembler.assembled()
    }
}
