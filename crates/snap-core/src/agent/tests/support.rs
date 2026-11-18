//! 시험 재료 한 자리: 임시 폴더 재료를 도구 시험 지원부에서 한 번만 가져옴
//!
//! 시험 본문이 제품 파일에 동거하지 않도록 이 트리에 둠

#![allow(dead_code, unreachable_pub)]

pub use crate::tools::tests::support::TempFolder;
