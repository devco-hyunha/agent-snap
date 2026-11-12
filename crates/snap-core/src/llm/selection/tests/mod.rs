//! 선택 키 시험 트리의 루트입니다(02-7 AC 1).
//!
//! 준비 골격(`support`)과 시험 본문(`client_build`·`keys`·`sampling`·`reasoning_extra`)을
//! 책임별로 나눈 트리고, 시험 본문은 제품 파일과 가까이 있는 이 트리에 둡니다
//! (dev-principles 9절).

#![allow(dead_code, unreachable_pub)]

mod client_build;
mod keys;
mod reasoning_extra;
mod sampling;
mod support;
