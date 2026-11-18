//! 기록·등록 파일 3종을 모은 폴더
//!
//! `record_fill`은 에이전트 모듈 안에서 `executor`가 쓰는 자리라 `pub(super)`로 둠

pub(super) mod record_fill;
pub mod registry;
pub mod truncate;
