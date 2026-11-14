//! 에이전트 루프 모듈: 책임별로 파일로 나누고 공개면을 한곳에서 내보냅니다.
//!
//! T6 F2에서 같은 관심사 파일을 `loop/`·`fallback/`·`record/`·`schema/`·`settings/`·`prompt/`로
//! 옮겼습니다(근거: PROJECT_RULES 34행, 같은 관심사 파일 둘 이상). 2안 텍스트 프로토콜 파일은
//! `text/`에 담았습니다(지시 조립·왕복·복원·검증·안내·기록·messages 줄·스키마 글자).
//! 파일 이름 두 개는 폴더 이름과 같아지면 clippy의 module-inception에 걸리므로 정정했습니다
//! (`prompt/assemble.rs`는 messages 조립, `fallback/connector.rs`는 폴백 연결 지점 표기).
//! `loop`은 Rust 예약어라 코드에서는 `r#loop`으로 씁니다.
//!
//! 루프 골격(`runner`)·왕복 상태(`loop_state`)·1안 왕복 판단(`openai_turn`)·스트림 드레인과 결과 채움(`loop_stream`)·첫 요청 조립(`loop_request`)·등록 목록(`registry`)·OpenAI 변환(`definitions`)·도구 실행(`executor`)·기록 채움(`record_fill`)·잘림(`truncate`)·messages 조립(`prompt/assemble`)·`AGENTS.md` 읽기(`prompt/agents_md`)·인자 JSON 복구(`fallback/args`)·프로파일(`settings/profile`)·설정 값(`settings/config`)·폴백 지점 판단(`fallback/connector`)·형식 변환과 상태 코드 표(`fallback/tool_call_form`)·미지원 조립(`fallback/unsupported`)·결과 타입(`schema/types`)이 각 파일의 책임이다.

pub mod executor;
pub mod text;

// T6 F2 이동: 같은 관심사 파일을 폴더로 옮겼다. 기존 이름과 경로는 이 창구에서 유지한다.
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
