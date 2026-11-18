//! 에이전트 루프 모듈.

pub mod executor;
pub mod text;

pub mod fallback;
pub mod r#loop;
pub mod prompt;
pub mod record;
pub mod schema;
pub mod settings;

pub use executor::{ToolRun, run_tool_calls};
pub use text::*;

pub use fallback::args::{self, recover_arguments};
pub use fallback::connector::{FallbackConnector, FallbackPoint, first_turn_fallback_point};
pub use fallback::tool_call_form::{
    self, ToolCallFormat, UNSUPPORTED_STATUS_CANDIDATES, is_unsupported_status,
};
pub use fallback::unsupported::{self, transport_failure, unsupported_point};
pub use r#loop::runner::AgentLoop;
pub use prompt::agents_md::{
    self, AGENTS_MD_FILE_NAME, AGENTS_MD_LONG_NOTICE, AgentsMd, DEFAULT_AGENTS_MD_MAX_BYTES,
    read_agents_md, read_agents_md_default,
};
pub use prompt::assemble::assemble_messages;
pub use record::registry::{self, ToolRegistry};
pub use record::truncate::{self, TRUNCATION_MARK, truncate_tool_result};
pub use schema::definitions::{self, registry_to_definitions};
pub use schema::types;
pub use schema::types::{
    LoopEnd, LoopError, LoopOutcome, LoopResult, LoopStartError, ToolCallRecord,
};
pub use settings::config::{self, DEFAULT_MAX_TURNS, DEFAULT_TOOL_RESULT_MAX_CHARS, LoopConfig};
pub use settings::profile::{self, ToolCallProfile};

#[cfg(test)]
mod tests;
