//! 2안 텍스트 프로토콜 시스템 프롬프트 조립 한 곳
//!
//! OpenAI `tools` 배열을 쓰지 못하는 로컬 모델에게 도구 이름·인자 형식·호출 예시를 글자로 알려줌

use crate::llm::ChatRequest;

use crate::agent::registry::ToolRegistry;

use super::schema_text::tool_line;

/// 1안 `tools` 배열을 빼고 시스템 프롬프트 끝에 2안 지시 블록을 붙임
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

/// 2안 지시 블록 앞머리. 모델이 답에 JSON 객체 하나만 쓰라고 지시
pub const TEXT_PROTOCOL_HEAD: &str = "도구 호출 형식";

/// 2안 지시 블록 끝부분. "JSON" 글자를 명시해 모델이 쓸 답을 정해줌
pub const TEXT_PROTOCOL_RULES: &str =
    "답에는 JSON 객체 하나만 써줘. 여러 도구를 한 번에 부르지 마.";

/// 한 번에 노출하는 도구로 권하는 상한
pub const TEXT_PROTOCOL_TOOL_GUIDE_LIMIT: usize = 20;

/// 2안 시스템 프롬프트 끝에 "도구 호출 형식" 블록을 붙임
///
/// 도구가 20개를 넘으면 등록 안내를 한 줄로 더함
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
