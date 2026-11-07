//! 재시도 판단 통합 테스트 타깃의 루트입니다.
//!
//! 함께 쓰는 지원(`common`)은 여기에서 선언하고, 실제 시험은 `decide` 파일에 둡니다.

pub mod common;

#[path = "retry/decide.rs"]
mod decide;
