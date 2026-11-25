//! 바이트 조각을 완성 줄로 재결합하는 상태 타입 자리.

use crate::llm::lines::split::decode;
use crate::llm::lines::split::split_complete_lines;
use crate::llm::lines::split::tail_line;
use crate::llm::split_error::SplitError;

/// 바이트 조각을 완성 줄로 재결합하는 상태.
#[derive(Debug, Default)]
pub struct Lines {
    // 처리하지 못한 바이트. 잘린 멀티바이트 문자와 줄 끝이 없는 꼬리를 함께 둠.
    pending: Vec<u8>,
}

impl Lines {
    /// 빈 줄 재결합 상태.
    pub fn new() -> Self {
        Self::default()
    }

    /// 바이트 조각을 넣고 완성 줄 목록을 받음.
    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<String>, SplitError> {
        self.pending.extend_from_slice(bytes);
        let source = std::mem::take(&mut self.pending);
        let (lines, tail) = split_complete_lines(&source);
        self.pending = tail;
        lines.into_iter().map(|line| decode(&line)).collect()
    }

    /// 끝까지 처리한 마지막 줄. 남은 바이트가 없으면 None.
    pub fn finish(&mut self) -> Result<Option<String>, SplitError> {
        let source = std::mem::take(&mut self.pending);
        let Some(line) = tail_line(&source) else {
            return Ok(None);
        };
        decode(line).map(Some)
    }
}
