//! 가짜 승인자 조립 재료를 파일별로 나눈 등록 자리
//!
//! 승인자를 쓰는 시험이 같은 조립을 다시 적지 않게 가져다 씀

#![allow(dead_code)]

#[path = "box_of.rs"]
mod box_of;
#[path = "counting.rs"]
mod counting;
#[path = "instant.rs"]
mod instant;
#[path = "waiting.rs"]
mod waiting;

pub(in crate::tools) use counting::counting_approver_with_reply;
pub(in crate::tools) use instant::{approved_approver, rejected_approver};
pub(in crate::tools) use waiting::waiting_approver_box;
