//! 전송 내부 항목을 보는 단위 시험 묶음입니다.
//!
//! 매핑 표·판단 반복·시간 주입처럼 제품 내부(`pub(crate)`) 항목을 직접 보는 시험만 이 트리에 둡니다.
//! 함께 쓰는 도우미는 바깥 공통 트리의 `support`에서 한 자리로 다시 내보냅니다.

pub(crate) use super::common::support::{
    client_at_with, connect_check_http, drain, retry_status_http, set_unit_sample, test_http,
};

mod connect_errors;
mod redact;
mod retry_map;
mod retry_run;
mod retry_status;
mod stream_disconnect;
