//! 03-2 에이전트 루프 통합 테스트 타깃의 루트입니다(spec AC 1~7).
//!
//! 함께 쓰는 지원(`common`)과 시험 파일을 한 자리에서 등록합니다.

pub mod common;

#[path = "agent_loop/profile.rs"]
mod profile;
#[path = "agent_loop/prompt_extra.rs"]
mod prompt_extra;
#[path = "agent_loop/registry.rs"]
mod registry;
#[path = "agent_loop/reply.rs"]
mod reply;
#[path = "agent_loop/support.rs"]
pub mod support;
#[path = "agent_loop/turns.rs"]
mod turns;
