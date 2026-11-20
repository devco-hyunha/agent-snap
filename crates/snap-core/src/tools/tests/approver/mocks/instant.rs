//! 즉시 답하는 가짜 승인자 재료: 승인·거부 창구를 만듦

use crate::tools::approval::request::{ApprovalFuture, ApproverBox};
use crate::tools::approver::{ApprovalReply, CommandApprover};

use super::box_of::box_of;

/// 즉시 돌려주는 가짜 승인자. 응답을 `ApprovalReply`로 고정해 시험이 같은 조립을 다시 쓰지 않게 함
struct MockApprover {
    reply: ApprovalReply,
}

impl CommandApprover for MockApprover {
    fn approve<'a>(&'a self, _summary: &'a str) -> ApprovalFuture<'a> {
        Box::pin(async move { self.reply })
    }
}

/// 즉시 승인을 돌려주는 가짜 승인자 창구
#[must_use]
pub(in crate::tools) fn approved_approver() -> ApproverBox {
    box_of(MockApprover {
        reply: ApprovalReply::Approved,
    })
}

/// 즉시 거부를 돌려주는 가짜 승인자 창구
#[must_use]
pub(in crate::tools) fn rejected_approver() -> ApproverBox {
    box_of(MockApprover {
        reply: ApprovalReply::Rejected,
    })
}
