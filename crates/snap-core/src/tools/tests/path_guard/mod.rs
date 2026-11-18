//! 경로 검사 시험 트리를 대상 모듈별로 나눔
//!
//! 제품 쪽이 `path_guard`·`secret_names`·`path_norm`·`link_guard` 네 파일로 나뉜 것에 맞춰 시험도 대상마다 한 파일로 둠
//! 판정 재료는 여러 파일이 함께 쓸 수 있게 한 자리에서 내보냄

#![allow(dead_code, unreachable_pub)]

mod boundary;
mod link;
mod normalization;
mod secret_names;

pub(in crate::tools) use boundary::assert_rejected;
