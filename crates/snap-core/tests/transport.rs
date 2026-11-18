//! 전송 계층 통합 시험 타깃의 루트.
//!
//! 지원(`common`)과 도우미 재내내기만 하고, 시험은 `cancel`·`restore`·`timeouts`·`usage` 파일에 분리.

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
