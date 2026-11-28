//! 생각 강도 허용값·설정 값 매핑 자리. `ReasoningEffort`의 강도 변환 로직만 담음.

use crate::llm::client::request::ReasoningEffort;

impl ReasoningEffort {
    /// 설정 값과 강도를 잇는 짝. 허용 강도 값과 이름 매핑의 한곳 정본.
    pub const PAIRS: [(Self, &'static str); 4] = [
        (Self::Off, "off"),
        (Self::Low, "low"),
        (Self::Medium, "medium"),
        (Self::High, "high"),
    ];

    /// 설정 값 문자열을 강도로 옮김. 목록 밖은 `None`.
    pub fn from_setting(value: &str) -> Option<Self> {
        Self::PAIRS
            .iter()
            .find(|(_, name)| *name == value)
            .map(|(effort, _)| *effort)
    }
}
