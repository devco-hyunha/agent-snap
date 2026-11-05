//! 전송 계층 통합 테스트 타깃의 루트입니다.
//!
//! 함께 쓰는 지원(`common`)과 도우미 재내내기만 여기에서 하고,
//! 실제 시험은 `cancel`·`restore`·`timeouts`·`usage` 파일에 분리해 둡니다.

pub mod common;

#[path = "transport/cancel.rs"]
mod cancel;
#[path = "transport/restore.rs"]
mod restore;
#[path = "transport/timeouts.rs"]
mod timeouts;
#[path = "transport/usage.rs"]
mod usage;

pub use common::support::{
    client_at_with, connect_check_http, drain, retry_status_http, test_http,
};
