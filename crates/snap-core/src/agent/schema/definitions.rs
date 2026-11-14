//! 도구 등록 목록을 OpenAI `tools` 배열 자리로 바꾸는 변환 한 곳입니다.
//!
//! 등록 목록 자체(`registry`)는 유지·조회 책임이고, 이 파일은 OpenAI 요청용 변환만 맡습니다(변경 이유 분리).
//! 03-3·03-6·03-8은 등록만 하고 이 변환을 씁니다.

use crate::llm::ToolDefinition;

/// 등록 목록을 OpenAI `tools` 배열 자리로 바꿉니다.
///
/// 03-1 예제 모양(`type: function`, `function.name/description/parameters`)을 그대로 만듭니다.
/// 본문 조립은 `llm/request_body.rs`가 같은 모양으로 보내므로, 변환만 이 파일이 맡습니다.
#[must_use]
pub fn registry_to_definitions(
    registry: &crate::agent::record::registry::ToolRegistry,
) -> Vec<ToolDefinition> {
    registry
        .tools()
        .iter()
        .map(|tool| ToolDefinition {
            name: tool.name().to_owned(),
            description: tool.description().to_owned(),
            parameters: tool.parameters_schema(),
        })
        .collect()
}
