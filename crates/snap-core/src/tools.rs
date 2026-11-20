//! 에이전트 도구를 모으는 모듈. 도구 구현 하나 = 파일 하나, 등록은 `agent/record/registry.rs`.
//!
//! 이 모듈은 구현과 경로 검사·표시 헬퍼만 둠.

pub mod args;
pub mod background;
pub mod check_background;
pub mod command;
pub mod display;
pub mod edit_file;
pub mod execute;
pub(crate) mod execute_bridge;
pub mod glob;
pub mod grep;
pub mod list_dir;
pub mod path;
pub mod process_tree;
pub mod read_file;
pub(crate) mod read_text;
pub mod ripgrep;
pub mod tool;
pub mod write_file;

// 지원 파일은 폴더로 묶음. 기존 경로(`tools::approver`·`tools::path_guard`·`tools::walk`)만 창구로 내보냄
pub mod approval;
pub use approval::approval_state;
pub use approval::approver;

pub use path::path_guard;

pub(crate) mod search;
pub(crate) use search::command_probe;
pub(crate) use search::glob_pattern;
pub(crate) use search::walk;

pub use args::parse_tool_args;
pub use command::{RunCommandSettings, RunCommandTool};
pub use edit_file::EditFileTool;
pub use glob::GlobTool;
pub use grep::GrepTool;
pub use list_dir::ListDirTool;
pub use path_guard::{PathGuardError, check_tool_path};
pub use read_file::ReadFileTool;
pub use write_file::WriteFileTool;

#[cfg(test)]
pub(crate) mod tests;
