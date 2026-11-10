//! 에이전트가 쓰는 도구(Tool 트레이트 구현)를 모으는 모듈입니다.
//!
//! 도구 구현 하나 = 파일 하나(PRO젝트 규칙 개발 규칙). 등록은 `agent/registry.rs`가 맡고,
//! 이 모듈은 구현과 경로 검사·표시 헬퍼만 둡니다.

pub mod args;
pub mod display;
pub mod edit_file;
pub mod glob;
pub mod grep;
pub(crate) mod link_guard;
pub mod list_dir;
pub mod path_guard;
pub(crate) mod path_norm;
pub mod read_file;
pub(crate) mod ripgrep;
pub(crate) mod search;
pub(crate) mod secret_names;
pub mod tool;
pub(crate) mod walk;
pub mod write_file;

pub use args::parse_tool_args;
pub use edit_file::EditFileTool;
pub use glob::GlobTool;
pub use grep::GrepTool;
pub use list_dir::ListDirTool;
pub use path_guard::{PathGuardError, check_tool_path};
pub use read_file::ReadFileTool;
pub use write_file::WriteFileTool;

#[cfg(test)]
mod tests;
