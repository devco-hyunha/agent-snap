//! messages를 조립하는 파일과 `AGENTS.md`를 읽는 파일 2종을 모은 폴더입니다(T6 F2 폴더 정리).
//!
//! 선언만 모였고, 파일 이름과 가시성은 그대로 두었습니다. messages 조립 파일은 폴더와
//! 같은 이름이 되면 clippy의 module-inception에 걸리므로 `assemble.rs`로 이름을 정정했습니다
//! (이 파일의 책임이 messages를 조립하는 일이라 이름에 맞습니다).

pub mod agents_md;
pub mod assemble;
