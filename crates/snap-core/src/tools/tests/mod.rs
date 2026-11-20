//! 파일 도구 시험 등록 자리
//!
//! 이 트리는 크레이트 안 항목을 보는 시험이라 `crate` 경로로만 봄

mod approved_context;
mod approver;
mod args;
mod background;
#[path = "edit_file.rs"]
mod edit_file;
mod execute_bridge;
#[path = "glob.rs"]
mod glob;
#[path = "grep.rs"]
mod grep;
#[path = "list_dir.rs"]
mod list_dir;
mod path_guard;
#[path = "process_tree.rs"]
mod process_tree;
#[path = "read_file.rs"]
mod read_file;
#[path = "ripgrep.rs"]
mod ripgrep;
mod run_command;
mod shell;
pub mod support;
#[path = "write_file.rs"]
mod write_file;
