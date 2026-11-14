//! 에이전트가 쓰는 도구(Tool 트레이트 구현)를 모으는 모듈입니다.
//!
//! 도구 구현 하나 = 파일 하나(프로젝트 규칙 개발 규칙). 등록은 `agent/record/registry.rs`가 맡고,
//! 이 모듈은 구현과 경로 검사·표시 헬퍼만 둡니다.

pub mod args;
pub mod display;
pub mod edit_file;
pub mod glob;
pub mod grep;
pub mod list_dir;
pub mod process_tree;
pub mod read_file;
pub(crate) mod read_text;
pub mod ripgrep;
pub(crate) mod run_command;
pub(crate) mod run_command_settings;
pub mod tool;
pub mod write_file;

// T6 F1 이동: 지원 파일을 폴더로 묶었다. 기존 경로(`tools::approver`·`tools::path_guard`·
// `tools::walk`)를 창구로 다시 내보낸다. 폴더 안에서 직접 쓰는 헬퍼까지 넓혀 내보내지 않는다.
pub mod approval;
pub use approval::approval_state;
pub use approval::approver;

pub mod path;
pub use path::path_guard;

pub(crate) mod search;
pub(crate) use search::command_probe;
pub(crate) use search::glob_pattern;
pub(crate) use search::walk;

pub use args::parse_tool_args;
pub use edit_file::EditFileTool;
pub use glob::GlobTool;
pub use grep::GrepTool;
pub use list_dir::ListDirTool;
pub use path_guard::{PathGuardError, check_tool_path};
pub use read_file::ReadFileTool;
pub use run_command::RunCommandTool;
pub use run_command_settings::RunCommandSettings;
pub use write_file::WriteFileTool;

#[cfg(test)]
pub(crate) mod tests;
