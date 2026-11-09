//! 폴백 연결 지점 판단: 03-4가 소비하는 전환 판단 값과 4x 판별만 담습니다(03-2 T2, spec AC 4).
//!
//! 2안 텍스트 프로토콜 조립·복원은 03-4의 책임입니다. 이 파일은 그쪽으로 넘길 판단 값과
//! 판별 규칙만 만듭니다(관심사 분리, dev-principles 5절). 상태 코드 판별은 가리기를 거친
//! 본문 글자를 보지 않습니다(D28).
//!
//! 커넥터 타입과 판단 값을 한 파일 안에서 역할별로 정리했습니다(리뷰 1차 N1 보정).
//! 커넥터 → 판단 값 → 상태 코드 표 → 판별 함수 순서로 두었습니다.

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

/// 03-4 폴백 경로로 넘길 전환 판단 값입니다. 03-2는 이 값만 만들고, 2안 조립은 하지 않습니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackPoint {
    /// 서버가 미지원 후보 4x 상태 코드로 답했습니다. 코드를 그대로 넘깁니다.
    UnsupportedStatus {
        /// 서버가 준 HTTP 상태 코드입니다.
        status: u16,
    },
    /// 첫 왕복에 도구 호출 없이 텍스트만 왔습니다. 2안 전환 기록 입력으로만 씁니다.
    NoToolCallText,
}

impl Display for FallbackPoint {
    /// 판단 근거를 한 줄로 출력합니다.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedStatus { status } => write!(f, "서버 상태 코드 {status}"),
            Self::NoToolCallText => f.write_str("호출 없는 첫 턴 텍스트"),
        }
    }
}

/// 미지원 후보로 보는 서버 상태 코드 목록입니다. 재시도 후보 목록(`retry/classify.rs`의
/// `RETRY_STATUS_WHITELIST`)과 같은 자리 규칙으로 한곳에 둡니다.
///
/// 호출 형식을 거부하는 서버가 보내는 상태를 골랐습니다(400·404·405·406·415·422·501).
/// 429와 5xx는 02-6 재시도 후보에 이미 있으므로 이곳에 두지 않습니다.
pub const UNSUPPORTED_STATUS_CANDIDATES: &[u16] = &[400, 404, 405, 406, 415, 422, 501];

/// 서버 상태 코드가 미지원 후보인지 판별합니다(상태 코드 표 한곳, 본문 글자 판별 금지).
#[must_use]
pub fn is_unsupported_status(status: u16) -> bool {
    UNSUPPORTED_STATUS_CANDIDATES.contains(&status)
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

/// 왕복이 1안(`tools`/`tool_calls`)으로 진행 중인지, 2안 텍스트 프로토콜로 전환했는지 나타내는 호출 형식입니다.
///
/// 루프는 1안으로만 보내고, 2안은 03-4가 채웁니다. 전환 기록은 03-4 AC 5가 소비합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallFormat {
    /// 1안 OpenAI 경로의 결과입니다. 03-2 루프가 만드는 기록은 항상 이 값입니다.
    OpenAi,
    /// 2안 텍스트 프로토콜의 결과입니다. 03-4가 기록을 만들 때만 채웁니다.
    Text,
}
