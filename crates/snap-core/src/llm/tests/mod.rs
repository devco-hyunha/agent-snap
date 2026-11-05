//! 제품 내부 항목을 보는 단위 테스트 트리의 루트입니다.
//!
//! 이 파일은 `snap-core` 라이브러리 안의 시험 전용 루트입니다. 제품 코드의 내부 항목을 그대로
//! 호출해야 하는 시험만 이 트리에 두고, 나머지는 바깥 시험(`crates/snap-core/tests/`)로 옮깁니다.
//! 함께 쓰는 지원은 바깥 공통 트리를 그대로 읽고 씁니다.

#[path = "../../../tests/common/mod.rs"]
pub mod common;

#[path = "snap_core_shim.rs"]
pub mod snap_core;

mod redact;
mod retry_unit;
mod transport_unit;
