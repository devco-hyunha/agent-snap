//! 스트림 결과 타입 자리. 합친 응답·사용량·원시 시각만 담음. 지표 계산은 없음.

use std::time::Instant;

use crate::llm::Delta;

/// 스트림이 끝난 뒤 받는 결과.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamOutcome {
    /// 끝까지 합친 하나의 응답.
    pub delta: Delta,

    /// 서버가 준 시각.
    pub timings: StreamTimings,
}

/// 요청 시작·첫 토큰·마지막 토큰의 원시 시각. 평균내지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamTimings {
    /// 요청을 시작한 시각.
    pub request_start: Instant,

    /// 첫 토큰을 받은 시각. 토큰이 없으면 없음.
    pub first_token: Option<Instant>,

    /// 마지막 토큰을 받은 시각. 토큰이 없으면 없음.
    pub last_token: Option<Instant>,
}
