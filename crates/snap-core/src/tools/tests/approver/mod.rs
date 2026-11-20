//! 승인자 호출 시험 트리를 책임별 파일로 나눈 등록 자리
//!
//! 트리가 크레이트 안 항목만 보므로 `#[path]`로 파일 이름을 그대로 연결

pub(in crate::tools) mod mocks;
#[path = "rejection.rs"]
mod rejection;
mod request;
