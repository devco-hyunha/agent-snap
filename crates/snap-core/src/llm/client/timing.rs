//! 요청별 원시 시각 타입입니다(평균 없음).

use std::time::Instant;

/// 요청별 원시 시각입니다(평균 없음).
///
/// `request_started`는 요청을 보내기 직전, `first_token`·`last_token`은
/// content·생각·도구 델타가 처음/마지막으로 온 시각입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamTiming {
    /// 요청을 보내기 시작한 시각입니다.
    pub request_started: Instant,
    /// 첫 토큰(내용 델타)이 온 시각입니다. 없으면 `None`입니다.
    pub first_token: Option<Instant>,
    /// 마지막 토큰이 온 시각입니다. 없으면 `None`입니다.
    pub last_token: Option<Instant>,
}
