//! execute 브리지 시험을 책임별 파일로 나눈 자리.
//!
//! 같은 조립을 쓰는 재료는 `shared.rs`에 한 자리로 두고, 크레이트 안 항목을 보는 시험으로 진행.

#[path = "approval.rs"]
mod approval;
#[path = "blocked.rs"]
mod blocked;
#[path = "length.rs"]
mod length;
#[path = "record.rs"]
mod record;
#[path = "roundtrip.rs"]
mod roundtrip;
pub mod shared;
#[path = "timeout.rs"]
mod timeout;
