//! 검색 지원 파일 4종을 모은 폴더입니다(T6 F1 폴더 정리).
//!
//! 각 파일의 가시성과 이름은 그대로 두고, 선언만 모읍니다. 내장 검색 파일은
//! 폴더와 같은 이름이 되면 clippy의 module-inception에 걸리므로 `builtin.rs`로
//! 자리를 정합니다.

pub(crate) mod builtin;
pub(crate) mod command_probe;
pub(crate) mod glob_pattern;
pub(crate) mod walk;
