//! 첫 요청 조립: 왕복을 시작할 요청 골격을 한 곳에서 준비
//!
//! 루프 골격(`loop/runner.rs`)은 왕복 반복만 담당하고, 이 파일은 첫 요청 조립만 담당

use crate::llm::ChatRequest;

use crate::agent::fallback::tool_call_form::{ToolCallFormat, start_format};
use crate::agent::prompt::agents_md;
use crate::agent::prompt::assemble::assemble_messages;
use crate::agent::schema::definitions::registry_to_definitions;
use crate::agent::schema::types::{LoopError, LoopStartError};
use crate::agent::settings::config::LoopConfig;
use crate::agent::text::protocol::apply_text_assembly;

/// 첫 요청 골격 생성. 등록 변환·프롬프트 조립·프로파일 합치기를 한곳에서 묶음
///
/// 등록 이름 중복은 요청을 보내기 전에 막음
pub(super) fn first_request(config: &LoopConfig) -> Result<ChatRequest, LoopError> {
    if let Some((name, _position)) = config.registry.find_duplicate_name() {
        return Err(LoopError::Start(LoopStartError::DuplicateToolName { name }));
    }
    let mut request = config.base.clone();
    // AGENTS.md 지시는 읽기 책임(`agents_md`)이 작업 폴더에서 읽고, 조립은 그대로 사용
    let agents_md =
        agents_md::read_agents_md(&config.context.working_dir, config.agents_md_max_bytes);
    request.messages = assemble_messages(
        &config.system_prompt,
        &agents_md.directives(),
        &config.user_message,
    );
    request.tools = registry_to_definitions(&config.registry);
    // `text` 선언은 감지 없이 2안 조립으로 진행(선언 우선)
    if matches!(start_format(config.tool_call_form), ToolCallFormat::Text) {
        apply_text_assembly(&mut request, &config.registry);
    }
    if let Some(profile) = &config.profile {
        profile.apply(&mut request).map_err(LoopError::Profile)?;
    }
    Ok(request)
}
