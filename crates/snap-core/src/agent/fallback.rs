use std::fmt::{self, Display, Formatter};

use crate::llm::TransportError;

use super::types::LoopError;

// 선언 타입과 루프 형식의 관계를 밝히는 자리입니다.
// `ToolCallForm`은 설정의 선언 타입(`crate::config::ToolCallForm`)이고, `ToolCallFormat`은 루프가
// 쓰는 왕복 형식 값입니다. 선언 지식을 이 한 파일에만 두고 루프(runner)가 같은 경로를 쓰게
// `pub use`로 열어 공개 재수출로 씁니다(E0603 보정).
pub use crate::config::ToolCallForm;

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

/// 선언 값(`tool_call_form`)이 2안으로 시작하는 선언인지 판별합니다(선언 우선 → 감지, D50 결정 3).
///
/// `text` 선언만 2안으로 시작하고, `openai`·`auto`는 1안으로 시작합니다. `auto`의 4x·2회 연속
/// 감지는 `fallback`의 다른 판별 함수들이 맡습니다.
#[must_use]
pub const fn starts_from_text_declaration(declared: ToolCallForm) -> bool {
    matches!(declared, ToolCallForm::Text)
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
/// 1안 기록은 03-2가, 2안 기록은 03-4가 채웁니다. 전환 기록은 03-4 AC 5가 소비합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallFormat {
    /// 1안 OpenAI 경로의 결과입니다. 03-2 루프가 만드는 기록은 항상 이 값입니다.
    OpenAi,
    /// 2안 텍스트 프로토콜의 결과입니다. 03-4가 기록을 만들 때만 채웁니다.
    Text,
}

/// 왕복 형식의 시작 값과 감지 여부를 정하는 선언(`crate::config::ToolCallForm`)을 루프 형식으로 옮깁니다(선언 우선, D50 결정 3).
///
/// `text` 선언만 2안으로 시작하고, `openai`·`auto`는 1안으로 시작합니다. 설정 타입을 그대로 받아
/// 루프가 쓰는 형식 상태로 바꾸는 자리이고, 같은 지식을 두 타입에 나누지 않습니다(DRY).
#[must_use]
pub const fn start_format(declared: ToolCallForm) -> ToolCallFormat {
    if starts_from_text_declaration(declared) {
        ToolCallFormat::Text
    } else {
        ToolCallFormat::OpenAi
    }
}

/// 선언이 감지를 쓰는 값인지 판별합니다(`auto`만 감지를 씁니다, D50 결정 3).
#[must_use]
pub const fn detects_switch(declared: ToolCallForm) -> bool {
    matches!(declared, ToolCallForm::Auto)
}

/// 4x 미지원 코드를 받았는지 확인해 판단 값을 뽑습니다. 폴백 판단이 아닌 전송 실패는 None입니다.
#[must_use]
pub fn unsupported_point(failure: &LoopError) -> Option<FallbackPoint> {
    let LoopError::Unsupported { point, .. } = failure else {
        return None;
    };
    Some(*point)
}

/// 전송 실패를 루프 오류로 바꿉니다. 4x 미지원 코드는 폴백 연결 지점 판단으로 채우고(AC 4),
/// 그 밖의 전송 실패는 코드를 보존해 그대로 넘깁니다(재전송은 02-6 한 층만 함, D13).
#[must_use]
pub fn transport_failure(error: &TransportError) -> LoopError {
    let transport = LoopError::transport(error);
    // 상태 코드 응답에서만 4x 표를 봅니다. 상태 코드가 없는 실패는 문구만 보존해 그대로 넘깁니다.
    let LoopError::Transport {
        reason,
        status: Some(status),
    } = transport
    else {
        return transport;
    };
    if !is_unsupported_status(status) {
        return LoopError::Transport {
            reason,
            status: Some(status),
        };
    }
    LoopError::unsupported(FallbackPoint::UnsupportedStatus { status })
}
