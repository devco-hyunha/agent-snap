//! 03-2 루프 시험 재료를 역할별 파일로 나눈 트리의 루트입니다.
//!
//! 함께 쓰는 골격과 본문 헬퍼(`support/`의 파일들)와 03-4 형식 전환 시험(`form_protocol/`)을
//! 한 자리에서 등록합니다.

pub mod common;

#[path = "agent_loop/agents_md.rs"]
mod agents_md;
#[path = "agent_loop/fallback.rs"]
mod fallback;
#[path = "agent_loop/form_protocol/mod.rs"]
mod form_protocol;
#[path = "agent_loop/profile.rs"]
mod profile;
#[path = "agent_loop/prompt_extra.rs"]
mod prompt_extra;
#[path = "agent_loop/registry.rs"]
mod registry;
#[path = "agent_loop/reply.rs"]
mod reply;
#[path = "agent_loop/support/mod.rs"]
pub mod support;
#[path = "agent_loop/turns.rs"]
mod turns;
