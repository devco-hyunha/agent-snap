//! 폴백 지점 타입과 표기, 텍스트 연속 판별만 담당하는 자리

use std::fmt::{self, Display, Formatter};

/// 폴백 연결 지점 커넥터. 오류를 소비하는 자리를 나타내는 표기 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FallbackConnector;

impl FallbackConnector {
    /// 오류를 소비하는 자리임을 밝히는 고정 표기 값
    pub const FALLBACK_CONNECTOR: &'static str = "03-4 폴백 연결 지점";
}

impl Display for FallbackConnector {
    /// 고정 표기 값을 그대로 출력
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(Self::FALLBACK_CONNECTOR)
    }
}

/// 폴백 경로로 넘길 전환 판단 값. 1안 판정 값만 만들고, 2안 판정 값과 2안 조립은 폴백이 채움
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

/// 1안 왕복에서 도구 호출 없는 텍스트가 2안 전환을 확정할 만큼 이어졌는지 판별
///
/// 확정은 2회 연속에서 함
pub const TEXT_STREAK_SWITCH_COUNT: u32 = 2;

/// 텍스트 왕복 개수가 2안 전환 확정에 닿았는지 판별
#[must_use]
pub const fn is_text_switch_streak(text_turns: u32) -> bool {
    text_turns >= TEXT_STREAK_SWITCH_COUNT
}

/// 첫 왕복(부름 없는 텍스트)이면 2안 전환 판단 입력을 만들고, 그 밖의 왕복이면 None
///
/// 조건은 "첫 왕복 + 텍스트 + 도구 호출 기록 없음" 한 가지만 확인
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
