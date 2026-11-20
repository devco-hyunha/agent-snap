//! 응답을 기다리는 가짜 승인자 재료: 만료·cancel 시험용 창구를 만듦

use crate::tools::approval::request::{ApprovalFuture, ApproverBox};
use crate::tools::approver::CommandApprover;

use super::box_of::box_of;

/// 응답을 기다리는 가짜 승인자(만료 시험용)
struct WaitingApprover;

impl CommandApprover for WaitingApprover {
    fn approve<'a>(&'a self, _summary: &'a str) -> ApprovalFuture<'a> {
        Box::pin(async move { std::future::pending().await })
    }
}

/// 응답을 기다리는 가짜 승인자 창구(만료·cancel 시험용)
#[must_use]
pub(in crate::tools) fn waiting_approver_box() -> ApproverBox {
    box_of(WaitingApprover)
}
