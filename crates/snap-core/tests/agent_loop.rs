//! 루프 시험 트리의 루트 — 골격·헬퍼·형식 전환 시험을 등록.

pub mod common;

#[path = "agent_loop/agents_md.rs"]
mod agents_md;
#[path = "agent_loop/approval.rs"]
mod approval;
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
