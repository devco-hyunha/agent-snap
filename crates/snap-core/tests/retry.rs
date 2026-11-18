//! 재시도 판단 통합 시험 타깃의 루트.
//!
//! 지원(`common`)은 여기서 선언하고, 시험은 `decide` 파일에 둠.

pub mod common;

#[path = "retry/decide.rs"]
mod decide;
