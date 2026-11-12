//! 첫 요청 조립: 왕복을 시작할 요청 골격을 한 곳에서 준비합니다(03-6 리뷰 SRP 지적).
//!
//! 루프 골격(`runner.rs`)은 왕복 반복만 맡고, 이 파일은 첫 요청 조립만 맡습니다(dev-principles 2절 SRP).

use crate::llm::ChatRequest;

use super::config::LoopConfig;
use super::definitions::registry_to_definitions;
use super::prompt::assemble_messages;
use super::text::protocol::apply_text_assembly;
use super::tool_call_form::{ToolCallFormat, start_format};
use super::types::{LoopError, LoopStartError};

/// 첫 요청 골격을 만듭니다. 등록 변환·프롬프트 조립·프로파일 합치기를 한곳에서 묶습니다.
///
/// 등록 이름 중복은 첫 요청을 만들기 전에 막습니다. 같은 이름이면 `find`가 앞의 도구만
/// 찾아 뒤의 도구가 조용히 가려지므로, 요청을 보내고 나서 막으면 결과가 어긋납니다.
pub(super) fn first_request(config: &LoopConfig) -> Result<ChatRequest, LoopError> {
    if let Some((name, _position)) = config.registry.find_duplicate_name() {
        return Err(LoopError::Start(LoopStartError::DuplicateToolName { name }));
    }
    let mut request = config.base.clone();
    // AGENTS.md 지시는 03-5 읽기 책임(`agents_md`)이 작업 폴더에서 읽고, 조립은 03-2 그대로입니다.
    let agents_md =
        super::agents_md::read_agents_md(&config.context.working_dir, config.agents_md_max_bytes);
    request.messages = assemble_messages(
        &config.system_prompt,
        &agents_md.directives(),
        &config.user_message,
    );
    request.tools = registry_to_definitions(&config.registry);
    // `text` 선언은 감지 없이 2안 조립으로 나갑니다(spec AC 7, 선언 우선).
    if matches!(start_format(config.tool_call_form), ToolCallFormat::Text) {
        apply_text_assembly(&mut request, &config.registry);
    }
    if let Some(profile) = &config.profile {
        profile.apply(&mut request).map_err(LoopError::Profile)?;
    }
    Ok(request)
}
