//! 도구 등록 목록을 OpenAI `tools` 배열 자리로 바꾸는 변환 한 곳
//!
//! 등록 목록 자체(`registry`)는 유지·조회 책임이고, 이 파일은 OpenAI 요청용 변환만 담당

use crate::llm::ToolDefinition;

/// 등록 목록을 OpenAI `tools` 배열 자리로 변환
///
/// 본문 조립은 `llm/request_body.rs`가 같은 모양으로 보내므로, 변환만 이 파일이 담당
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
