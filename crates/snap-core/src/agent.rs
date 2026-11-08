//! 에이전트 루프 모듈: 책임별로 파일로 나누고 공개면을 한곳에서 내보냅니다.
//!
//! 루프(`runner`)·등록 목록(`registry`)·OpenAI 변환(`definitions`)·도구 실행(`executor`)·잘림(`truncate`)·
//! messages 조립(`prompt`)·인자 JSON 복구(`args`)·프로파일(`profile`)·설정 값(`config`)·
//! 결과 타입(`types`)이 각 파일의 책임입니다(dev-principles 2절 SRP).

pub mod args;
pub mod config;
pub mod definitions;
pub mod executor;
pub mod profile;
pub mod prompt;
pub mod registry;
pub mod runner;
pub mod truncate;
pub mod types;

pub use args::recover_arguments;
pub use config::{DEFAULT_MAX_TURNS, DEFAULT_TOOL_RESULT_MAX_CHARS, LoopConfig};
pub use definitions::registry_to_definitions;
pub use executor::{ToolRun, run_tool_calls};
pub use profile::ToolCallProfile;
pub use prompt::{EMPTY_AGENTS_MD, assemble_messages};
pub use registry::ToolRegistry;
pub use runner::AgentLoop;
pub use truncate::{TRUNCATION_MARK, truncate_tool_result};
pub use types::{LoopEnd, LoopError, LoopOutcome, LoopResult, LoopStartError, ToolCallRecord};
