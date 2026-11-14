//! 선언→루프 형식 변환과 미지원 상태 코드 표만 맡는 자리입니다(03-6 R1 감사 분리).
//!
//! 책임 나눔: 타입·표기·연속 판별은 `connector.rs`, 판단 값 조립은 `unsupported.rs`,
//! 표와 형식 변환은 이 파일입니다. 조립 코드는 두지 않습니다(dev-principles 2절 SRP).

use crate::config::ToolCallForm;

// `ToolCallForm`은 설정의 선언 타입(`crate::config::ToolCallForm`)이고, `ToolCallFormat`은 루프가
// 쓰는 왕복 형식 값입니다. 왕복 형식 값을 쓰는 쪽은 이 파일의 정의를 쓰고, 선언 타입은
// `crate::config` 경로를 그대로 씁니다(중중 재수출을 두지 않아 한 지식만 다룹니다).

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

/// 선언 값(`tool_call_form`)이 2안으로 시작하는 선언인지 판별합니다(선언 우선 → 감지, D50 결정 3).
///
/// `text` 선언만 2안으로 시작하고, `openai`·`auto`는 1안으로 시작합니다. `auto`의 4x·2회 연속
/// 감지는 `fallback`과 `unsupported`의 다른 판별 함수들이 맡습니다.
#[must_use]
pub const fn starts_from_text_declaration(declared: ToolCallForm) -> bool {
    matches!(declared, ToolCallForm::Text)
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
