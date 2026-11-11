//! 03-2 루프 시험 재료를 역할별 파일로 나눈 트리의 루트입니다.
//!
//! 모의 서버 본문(`streams`)·가짜 도구와 등록 목록(`tools`)·설정 골격(`setup`)을 한 자리에서
//! 내보냅니다. 시험 파일들은 `crate::support` 경로 그대로 쓰므로 호출부는 변하지 않습니다.

#![allow(dead_code, unreachable_pub)]

pub mod setup;
pub mod streams;
pub mod tools;

pub use setup::{
    base_config, base_config_with, cancel_token, form_config, loop_config, loop_config_base,
    test_working_dir,
};
pub use streams::{content_stream, empty_stream, tool_call_stream, tool_calls_stream};
pub use tools::{
    ApprovalTool, FixedTool, registry_with_approval, registry_with_fixed,
    registry_with_fixed_twice, registry_with_list_dir, registry_with_named_fixed,
};
