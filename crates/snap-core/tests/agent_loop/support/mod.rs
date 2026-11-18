//! 루프 시험 재료 트리의 루트.
//!
//! 모의 서버 본문·가짜 도구와 등록 목록·설정 골격 내보내기.

#![allow(dead_code, unreachable_pub)]

pub mod setup;
pub mod streams;
pub mod tools;
pub mod workdir;

pub use setup::{
    agents_md_loop_config, base_config, base_config_with, cancel_token, form_config, loop_config,
    loop_config_base, test_working_dir,
};
pub use streams::{content_stream, empty_stream, tool_call_stream, tool_calls_stream};
pub use tools::{
    ApprovalTool, CountingApprover, FixedTool, counting_approver, loop_config_with_approver,
    registry_with_approval, registry_with_fixed, registry_with_fixed_twice, registry_with_list_dir,
    registry_with_named_fixed,
};
pub use workdir::LoopTempWorkdir;
