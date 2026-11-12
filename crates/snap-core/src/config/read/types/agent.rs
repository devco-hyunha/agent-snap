//! `agent` 섹션 설정 값 타입만 둡니다(03-6 T1, D22).
//!
//! 기본값 상수는 부모(`types/mod.rs`)에 두고, 여기서는 타입 선언만 다룹니다(02 설정 읽기
//! 규칙 — `http`·`model`과 같은 자리). 빠진 값은 [`AgentSettings::default`]가 채웁니다.

use std::time::Duration;

use super::{DEFAULT_APPROVAL_DEADLINE_SECONDS, DEFAULT_RUN_COMMAND_DEADLINE_SECONDS};

/// 에이전트 루프가 쓰는 시간 값입니다(`agent` 섹션, D22).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentSettings {
    /// `run_command` 실행 시간 상한입니다. 빠진 값은 기본 600초입니다(D22).
    pub run_command_seconds: Duration,
    /// 승인 응답을 기다리는 시간 상한입니다. 빠진 값은 기본 300초입니다(D22).
    pub approval_seconds: Duration,
}

impl Default for AgentSettings {
    fn default() -> Self {
        Self {
            run_command_seconds: Duration::from_secs(DEFAULT_RUN_COMMAND_DEADLINE_SECONDS),
            approval_seconds: Duration::from_secs(DEFAULT_APPROVAL_DEADLINE_SECONDS),
        }
    }
}
