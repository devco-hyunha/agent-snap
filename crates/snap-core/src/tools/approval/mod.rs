//! 승인자 트레이트와 상태 관리 자리를 묶은 폴더
//!
//! `approver`이 응답·승인자 트레이트 창구, `request`가 승인 대기, `rejection_text`가 거절 문구, `settings`가 승인 대기 상한

pub mod approval_state;
pub mod approver;
pub mod rejection_text;
pub mod request;
pub mod settings;

pub use request::{ApprovalFuture, ApproverBox, request_approval};
pub use settings::ApprovalSettings;
