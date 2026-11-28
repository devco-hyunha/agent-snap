//! 델타 누적 상태 타입 자리. 합치는 필드와 누적 칸만 담음.
//!
//! 필드를 붙이고 교체하는 계산은 `merge`가 맡음.

use crate::llm::delta::chunk_error::ChunkError;
use crate::llm::delta::chunk_types::ChunkDelta;
use crate::llm::delta::merge::apply_chunk_fields;
use crate::llm::delta::parse::chunk_delta_of;
use crate::llm::delta::parse::parse_chunk;
use crate::llm::delta::types::Delta;
use serde_json::Value;

/// 청크 델타를 하나의 응답으로 합치는 누적 상태.
#[derive(Debug, Default)]
pub struct DeltaState {
    delta: Delta,
}

impl DeltaState {
    /// 빈 누적 상태.
    pub fn new() -> Self {
        Self::default()
    }

    /// payload JSON 문자열 하나를 받아 응답에 합침.
    pub fn apply(&mut self, payload: &str) -> Result<(), ChunkError> {
        let chunk = parse_chunk(payload)?;
        self.apply_chunk(chunk);
        Ok(())
    }

    /// 해석된 payload JSON 값을 응답에 합침. 스트림 처리가 파싱을 한 번으로 끝내기 위한 창구.
    pub fn apply_value(&mut self, value: &Value) {
        self.apply_chunk(chunk_delta_of(value));
    }

    /// 지금까지 합친 하나의 응답 복사.
    pub fn current(&self) -> Delta {
        self.delta.clone()
    }

    /// 해석된 델타 필드 묶음을 응답에 합침.
    pub fn apply_chunk(&mut self, chunk: ChunkDelta) {
        apply_chunk_fields(&mut self.delta, &chunk);
    }

    /// 끝까지 합친 하나의 응답.
    pub fn finish(self) -> Delta {
        self.delta
    }
}
