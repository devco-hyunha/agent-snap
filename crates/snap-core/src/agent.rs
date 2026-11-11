//! 에이전트 루프 모듈: 책임별로 파일로 나누고 공개면을 한곳에서 내보냅니다.
//!
//! 루프 골격(`runner`)·왕복 상태(`loop_state`)·1안 왕복 판단(`openai_turn`)·스트림 드레인과
//! 결과 채움(`loop_stream`)·등록 목록(`registry`)·OpenAI 변환(`definitions`)·도구 실행(`executor`)·
//! 잘림(`truncate`)·messages 조립(`prompt`)·`AGENTS.md` 읽기(`agents_md`)·인자 JSON 복구(`args`)·프로파일(`profile`)·설정 값(`config`)·
//! 폴백 연결 지점 판단(`fallback`)·결과 타입(`types`)이 각 파일의 책임이고,
//! 2안 텍스트 프로토콜 파일들은 `text/` 폴더에 담았습니다(지시 조립·왕복·복원·검증·안내·기록·messages 줄).

pub mod agents_md;
pub mod args;
pub mod config;
pub mod definitions;
pub mod executor;
pub mod fallback;
pub mod loop_state;
pub mod loop_stream;
pub mod openai_turn;
pub mod profile;
pub mod prompt;
pub mod registry;
pub mod runner;
pub mod text;
pub mod truncate;
pub mod types;

pub use agents_md::{
    AGENTS_MD_FILE_NAME, AGENTS_MD_LONG_NOTICE, AgentsMd, DEFAULT_AGENTS_MD_MAX_BYTES,
    read_agents_md, read_agents_md_default,
};
pub use args::recover_arguments;
pub use config::{DEFAULT_MAX_TURNS, DEFAULT_TOOL_RESULT_MAX_CHARS, LoopConfig};
pub use definitions::registry_to_definitions;
pub use executor::{ToolRun, run_tool_calls};
pub use fallback::{
    FallbackConnector, FallbackPoint, ToolCallFormat, UNSUPPORTED_STATUS_CANDIDATES,
    first_turn_fallback_point, is_unsupported_status,
};
pub use profile::ToolCallProfile;
pub use prompt::assemble_messages;
pub use registry::ToolRegistry;
pub use runner::AgentLoop;
pub use text::*;
pub use truncate::{TRUNCATION_MARK, truncate_tool_result};
pub use types::{LoopEnd, LoopError, LoopOutcome, LoopResult, LoopStartError, ToolCallRecord};

#[cfg(test)]
mod tests;
