//! 루프 골격 파일 5종을 모은 폴더
//!
//! `loop`은 Rust 예약어라 바깥 참조는 `r#loop`로 사용

pub(crate) mod loop_request;
pub(crate) mod loop_state;
pub(crate) mod loop_stream;
pub(crate) mod openai_turn;
pub mod runner;
