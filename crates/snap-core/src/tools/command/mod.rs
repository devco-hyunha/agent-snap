//! 셸 명령 도메인: 도구 창구·승인 후 실행·파이프 읽기·시간 상한 값·결과 문구
//!
//! 도구 창구는 인자·승인·비밀 이름 검사만 하고, 실행 경로는 승인 표식이 채워진 명령만 다룸

pub mod settings;
pub mod tool;

mod approve;
pub(crate) mod pipes;
mod result_text;
mod secret_name;
mod tree_note;

pub use settings::RunCommandSettings;
pub use tool::RunCommandTool;
