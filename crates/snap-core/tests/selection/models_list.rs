//! 모델 목록 조회 시험 루트입니다.
//!
//! 하네스 10절·dev-principles 9절에 따라 시험을 관심사별로 파일을 나눴다(SRP).
//! - `ids_read`: 서버에서 id를 읽어 오는 시험
//! - `format_error`: 형식 오류를 내고 키·주소·본문을 가리는 시험
//! - `cancel`: 목록 조회의 취소 시험
//! - `timeouts`: 목록 조회의 시간 초과 시험
//! - `list_failure_keeps_chat`: 목록 조회 실패가 채팅 요청을 막지 않음
//! - `support`: 시험들이 같이 쓰는 조립

#[path = "models_list/support.rs"]
pub mod support;

#[path = "models_list/ids_read.rs"]
mod ids_read;

#[path = "models_list/format_error.rs"]
mod format_error;

#[path = "models_list/cancel.rs"]
mod cancel;

#[path = "models_list/timeouts.rs"]
mod timeouts;

#[path = "models_list/list_failure_keeps_chat.rs"]
mod list_failure_keeps_chat;
