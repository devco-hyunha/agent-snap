//! 폴백 지점 타입과 표기, 텍스트 연속 판별만 맡는 자리입니다(03-6 R1 감사 분리).
//!
//! 책임 나눔: 표(상태 코드)·형식 변환은 `tool_call_form.rs`, 판단 값 조립은 `unsupported.rs`,
//! 타입·표기·연속 판별은 이 파일입니다(dev-principles 2절 SRP).

use std::fmt::{self, Display, Formatter};

/// 폴백 연결 지점 커넥터입니다. 타입 하나가 "03-4가 이 오류를 소비하는 자리"를 나타내는
/// 값이 하나인 표기 타입입니다(리뷰 1차 N3, 하네스 1절 어투).
///
/// 표기할 폴백 지점이 있을 때만 쓰고, 지점이 없으면 `Option`의 `None`으로 "폴백 지점 없음"을
/// 타입으로 표현합니다(03-4 소비 시 넓혀 쓰는 자리, 리뷰 1차 N1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FallbackConnector;

impl FallbackConnector {
    /// 03-4가 이 오류를 소비하는 자리임을 밝히는 고정 표기 값입니다.
    pub const FALLBACK_CONNECTOR: &'static str = "03-4 폴백 연결 지점";
}

impl Display for FallbackConnector {
    /// 고정 표기 값을 그대로 출력합니다.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(Self::FALLBACK_CONNECTOR)
    }
}

/// 03-4 폴백 경로로 넘길 전환 판단 값입니다. 03-2는 1안 판정 값만 만들고, 03-4가 2안 판정 값과
/// 2안 조립을 채웁니다(전환 판단 한 파일의 책임, dev-principles 2절 SRP).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackPoint {
    /// 서버가 미지원 후보 4x 상태 코드로 답했습니다. 코드를 그대로 넘깁니다.
    UnsupportedStatus {
        /// 서버가 준 HTTP 상태 코드입니다.
        status: u16,
    },
    /// 첫 왕복에 도구 호출 없이 텍스트만 왔습니다. 2안 전환 기록 입력으로만 씁니다.
    NoToolCallText,
    /// 1안 왕복에서 도구 호출 없는 텍스트가 2번 이어졌습니다. 2안 전환 확정의 근거 값입니다(03-4, D50 결정 2).
    TextStreak,
}

impl Display for FallbackPoint {
    /// 판단 근거를 한 줄로 출력합니다.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedStatus { status } => write!(f, "서버 상태 코드 {status}"),
            Self::NoToolCallText => f.write_str("호출 없는 첫 턴 텍스트"),
            Self::TextStreak => f.write_str("호출 없는 텍스트 2회 연속"),
        }
    }
}

/// 1안 왕복에서 도구 호출 없는 텍스트가 2안 전환을 확정할 만큼 이어졌는지 판별합니다(D50 결정 2).
///
/// 텍스트 왕복 개수만 받고, 확정은 2회 연속에서 합니다. 1회는 전환이 아니라 후보 기록이고,
/// 그 기록 값은 루프가 `first_turn_fallback_point`로 만듭니다(이 판별은 2회 확정만 봅니다).
pub const TEXT_STREAK_SWITCH_COUNT: u32 = 2;

/// 텍스트 왕복 개수가 2안 전환을 확정할 만큼 이어졌는지 판별합니다(2회 연속, D50 결정 2).
///
/// 1회의 후보 기록 값은 루프가 `first_turn_fallback_point`로 만들기에, 1회 전용 판별 함수는 두지
/// 않습니다. 1회만 보는 판별은 루프에 연결되지 않는 죽은 코드입니다(리뷰 1차 minor 2,
/// dev-principles 4절 YAGNI·10절 미사용 코드 삭제).
#[must_use]
pub const fn is_text_switch_streak(text_turns: u32) -> bool {
    text_turns >= TEXT_STREAK_SWITCH_COUNT
}

/// 첫 왕복(부름 없는 텍스트)이면 2안 전환 판단 입력을 만들고, 그 밖의 왕복이면 None입니다.
///
/// 조건은 "첫 왕복 + 텍스트 + 도구 호출 기록 없음" 한 가지만 봅니다(할 일 6). 여러 번 반복 감지는
/// 세션 단위라 03-4가 기록으로 판단하고, 이 함수는 첫 판정 한 종류만 만듭니다.
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
