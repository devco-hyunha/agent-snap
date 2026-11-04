//! 스트림 정상 종료 뒤 남기는 사용량·시각 타입입니다.

use crate::llm::client::timing::StreamTiming;
use crate::llm::delta::Usage;

/// 스트림이 정상으로 끝난 뒤 돌려주는 사용량·시각입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamOutcome {
    /// 서버가 준 사용량입니다. 없으면 `None`(없음)입니다.
    pub usage: Option<Usage>,
    /// 요청 시작·첫 토큰·마지막 토큰 시각입니다.
    pub timing: StreamTiming,
}
