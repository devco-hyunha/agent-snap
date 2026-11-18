//! 제품 내부 항목을 보는 단위 시험 트리의 루트.
//!
//! 내부 항목을 직접 호출해야 하는 시험만 이 트리에 둠. 지원은 바깥 공통 트리를 읽음.

#[path = "../../../tests/common/mod.rs"]
pub mod common;

#[path = "snap_core_shim.rs"]
pub mod snap_core;

mod redact;
mod retry_unit;
mod transport_unit;
