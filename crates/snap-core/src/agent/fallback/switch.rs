//! 폴백 전환 판단 값과 텍스트 연속 판별

use std::fmt::{self, Display, Formatter};

/// 폴백 경로로 넘길 전환 판단 값
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackPoint {
    /// 서버가 미지원 후보 4x 상태 코드로 응답. 코드를 그대로 넘김
    UnsupportedStatus {
        /// 서버가 준 HTTP 상태 코드
        status: u16,
    },
    /// 첫 왕복에 도구 호출 없이 텍스트만 옴. 2안 전환 기록 입력으로만 사용
    NoToolCallText,
    /// 1안 왕복에서 도구 호출 없는 텍스트가 2번 이어짐. 2안 전환 확정의 근거 값
    TextStreak,
}

impl Display for FallbackPoint {
    /// 판단 근거를 한 줄로 출력
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedStatus { status } => write!(f, "서버 상태 코드 {status}"),
            Self::NoToolCallText => f.write_str("호출 없는 첫 턴 텍스트"),
            Self::TextStreak => f.write_str("호출 없는 텍스트 2회 연속"),
        }
    }
}

/// 2안 전환을 확정하는 연속 텍스트 횟수
pub const TEXT_STREAK_SWITCH_COUNT: u32 = 2;

/// 텍스트 왕복 개수가 2안 전환 확정에 닿았는지 판별
#[must_use]
pub const fn is_text_switch_streak(text_turns: u32) -> bool {
    text_turns >= TEXT_STREAK_SWITCH_COUNT
}

/// 첫 왕복의 호출 없는 텍스트를 전환 후보로 판별
#[must_use]
pub const fn first_turn_fallback_point(
    has_content: bool,
    turns: u32,
    has_records: bool,
) -> Option<FallbackPoint> {
    if !has_content || turns != 1 || has_records {
        return None;
    }
    Some(FallbackPoint::NoToolCallText)
}
