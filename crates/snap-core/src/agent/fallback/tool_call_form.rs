//! 선언→루프 형식 변환과 미지원 상태 코드 표만 담당하는 자리

use crate::config::ToolCallForm;

/// 미지원 후보 서버 상태 코드 목록. 재시도 후보 목록과 같은 규칙으로 한곳에 둠
///
/// 429와 5xx는 재시도 후보에 이미 있으므로 여기 두지 않음
pub const UNSUPPORTED_STATUS_CANDIDATES: &[u16] = &[400, 404, 405, 406, 415, 422, 501];

/// 서버 상태 코드가 미지원 후보인지 판별(상태 코드 표 한곳, 본문 글자 판별 금지)
#[must_use]
pub fn is_unsupported_status(status: u16) -> bool {
    UNSUPPORTED_STATUS_CANDIDATES.contains(&status)
}

/// 선언 값(`tool_call_form`)이 2안으로 시작하는 선언인지 판별(선언 우선 → 감지)
#[must_use]
pub const fn starts_from_text_declaration(declared: ToolCallForm) -> bool {
    matches!(declared, ToolCallForm::Text)
}

/// 왕복이 1안(`tools`/`tool_calls`)으로 진행 중인지, 2안 텍스트 프로토콜로 전환했는지 나타내는 호출 형식
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallFormat {
    /// 1안 OpenAI 경로의 결과. 루프가 만드는 기록은 항상 이 값
    OpenAi,
    /// 2안 텍스트 프로토콜의 결과. 기록을 만들 때만 채움
    Text,
}

/// 왕복 형식의 시작 값과 감지 여부를 정하는 선언(`crate::config::ToolCallForm`)을 루프 형식으로 이동(선언 우선)
///
/// 설정 타입을 그대로 받아 루프가 쓰는 형식 상태로 바꾸는 자리
#[must_use]
pub const fn start_format(declared: ToolCallForm) -> ToolCallFormat {
    if starts_from_text_declaration(declared) {
        ToolCallFormat::Text
    } else {
        ToolCallFormat::OpenAi
    }
}

/// 선언이 감지를 쓰는 값인지 판별(`auto`만 감지 사용)
#[must_use]
pub const fn detects_switch(declared: ToolCallForm) -> bool {
    matches!(declared, ToolCallForm::Auto)
}
