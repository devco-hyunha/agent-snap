//! 스트림 정상 종료 뒤 남기는 사용량·시각 타입.

use crate::llm::client::timing::StreamTiming;
use crate::llm::codec::delta::Usage;

/// 스트림 정상 종료 뒤 돌려주는 사용량·시각.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamOutcome {
    /// 서버가 준 사용량. 없으면 `None`.
    pub usage: Option<Usage>,
    /// 요청 시작·첫 토큰·마지막 토큰 시각.
    pub timing: StreamTiming,
}
