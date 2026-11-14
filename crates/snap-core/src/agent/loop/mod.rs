//! 루프 골격 파일 5종을 모은 폴더입니다(T6 F2 폴더 정리).
//!
//! 선언만 모였고, 가시성은 크레이트 안 사용 범위를 그대로 유지했습니다.
//! `loop`은 Rust 예약어라 바깥 참조는 `r#loop`로 씁니다.
//! 왕복 판단 4종과 첫 요청 조립 1종은 루프 안과 에이전트 안에서만 쓰는 값이라
//! `pub(crate)`로 두고, `runner`만 바깥에 공개합니다.

pub(crate) mod loop_request;
pub(crate) mod loop_state;
pub(crate) mod loop_stream;
pub(crate) mod openai_turn;
pub mod runner;
