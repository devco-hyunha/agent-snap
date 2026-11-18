//! 요청별 원시 시각 타입(평균 없음).

use std::time::Instant;

/// 요청별 원시 시각(평균 없음).
///
/// `request_started`는 보내기 직전, `first_token`·`last_token`은 델타 처음/마지막 시각.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamTiming {
    /// 요청을 보내기 시작한 시각.
    pub request_started: Instant,
    /// 첫 토큰(내용 델타)이 온 시각. 없으면 `None`.
    pub first_token: Option<Instant>,
    /// 마지막 토큰이 온 시각. 없으면 `None`.
    pub last_token: Option<Instant>,
}
