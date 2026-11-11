//! 03-5 시험 재료 한 자리: 임시 폴더 재료를 도구 시험 지원부에서 한 번만 가져옵니다(DRY).
//!
//! 시험 본문이 제품 파일에 동거하지 않도록 이 트리에 두고, 같은 재료를 시험마다 새로 만들지 않습니다.
//! 임시 폴더는 만들기·끝에 스스로 지우는 정리 순서가 정해져 있는 도구 시험 지원부를 그대로 씁니다.

#![allow(dead_code, unreachable_pub)]

pub use crate::tools::tests::support::TempFolder;
