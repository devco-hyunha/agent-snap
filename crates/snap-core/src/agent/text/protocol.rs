//! 2안 텍스트 프로토콜 시스템 프롬프트 조립 한 곳입니다(03-4 spec AC 2, D50 결정 8).
//!
//! OpenAI `tools` 배열을 쓰지 못하는 로컬 모델에게 도구 이름·인자 형식·호출 예시를 글자로
//! 알려줍니다. 시스템 프롬프트 끝에 "도구 호출 형식" 블록을 붙이고, 지시문에 "JSON" 글자를
//! 명시합니다(12번 문서 8절 업계 관례).
//!
//! 프롬프트 앞머리·사용자 줄을 만드는 일은 `prompt.rs`의 책임이고, 이 파일은 2안 지시 블록만
//! 만듭니다(관심사 분리, dev-principles 5절). 스키마에서 글자를 뽑는 세부 읽기는
//! `schema_text.rs`의 책임입니다(dev-principles 2절 SRP).

use crate::llm::ChatRequest;

use crate::agent::registry::ToolRegistry;

use super::schema_text::tool_line;

/// 1안 `tools` 배열을 빼고 시스템 프롬프트 끝에 2안 지시 블록을 붙입니다(전환 조립, D50 결정 1).
pub fn apply_text_assembly(request: &mut ChatRequest, registry: &ToolRegistry) {
    request.tools.clear();
    let block = text_protocol_block(registry);
    if let Some(system) = request
        .messages
        .iter_mut()
        .find(|message| message.role == "system")
    {
        system.content.push('\n');
        system.content.push_str(&block);
    }
}

/// 2안 지시 블록 앞머리입니다. 모델이 답에 JSON 객체 하나만 쓰라고 지시합니다(spec AC 2).
pub const TEXT_PROTOCOL_HEAD: &str = "도구 호출 형식";

/// 2안 지시 블록 끝부분입니다. "JSON" 글자를 명시해 모델이 쓸 답을 정해줍니다(D50 결정 8).
pub const TEXT_PROTOCOL_RULES: &str =
    "답에는 JSON 객체 하나만 써줘. 여러 도구를 한 번에 부르지 마.";

/// 한 번에 노출하는 도구로 권하는 상한입니다(OpenAI 공식 안내, 12번 문서 8절, D50 결정 8).
pub const TEXT_PROTOCOL_TOOL_GUIDE_LIMIT: usize = 20;

/// 2안 시스템 프롬프트 끝에 "도구 호출 형식" 블록을 붙입니다(spec AC 2).
///
/// 등록한 도구 전부를 넣습니다. 한 도구당 3줄(이름과 설명, 인자 필드와 타입, 호출 예시 JSON)로
/// 쓰고, 도구가 20개를 넘으면 등록 안내를 한 줄로 더합니다(D50 결정 8).
///
/// ```
/// use snap_core::agent::{text_protocol_block, ToolRegistry};
/// let registry = ToolRegistry::with_agent_tools();
/// let block = text_protocol_block(&registry);
/// assert!(block.contains("read_file"));
/// assert!(block.contains("JSON"));
/// ```
#[must_use]
pub fn text_protocol_block(registry: &ToolRegistry) -> String {
    let mut block = String::new();
    block.push_str(TEXT_PROTOCOL_HEAD);
    block.push('\n');
    for tool in registry.tools() {
        block.push_str(&tool_line(
            tool.name(),
            tool.description(),
            &tool.parameters_schema(),
        ));
        block.push('\n');
    }
    block.push_str(TEXT_PROTOCOL_RULES);
    if registry.tools().len() > TEXT_PROTOCOL_TOOL_GUIDE_LIMIT {
        block.push('\n');
        block.push_str("도구 등록 안내: 한 번에 노출하는 도구는 20개 미만으로 줄여줘.");
    }
    block
}
