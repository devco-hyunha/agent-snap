//! 03-3 파일 도구 시험 등록 자리입니다(제품 소스에 시험 본문을 두지 않습니다).
//!
//! 책임별 시험 파일 하나씩: 경로 검사·읽기·쓰기·편집·목록·이름검색·본문검색.
//! 이 트리는 크레이트 안 항목을 보는 시험이므로 `crate` 경로로만 봅니다(개발 의존과
//! 따로 놀아 타입 두 벌이 생기는 일을 피합니다).

#[path = "approver.rs"]
mod approver;
#[path = "edit_file.rs"]
mod edit_file;
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
#[path = "run_command.rs"]
mod run_command;
pub mod support;
#[path = "write_file.rs"]
mod write_file;
