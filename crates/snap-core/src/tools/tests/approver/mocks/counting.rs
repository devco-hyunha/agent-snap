//! 호출을 세고 정한 응답을 순번대로 돌려주는 가짜 승인자 재료

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::tools::approval::request::{ApprovalFuture, ApproverBox};
use crate::tools::approver::{ApprovalReply, CommandApprover};

use super::box_of::box_of;

/// 응답을 정해진 순번대로 돌려주고 부른 횟수를 세는 가짜 승인자
struct CountingApprover {
    calls: Arc<AtomicUsize>,
    replies: Vec<ApprovalReply>,
}

impl CommandApprover for CountingApprover {
    fn approve<'a>(&'a self, _summary: &'a str) -> ApprovalFuture<'a> {
        let index = self.calls.fetch_add(1, Ordering::Relaxed);
        let reply = self
            .replies
            .get(index)
            .copied()
            .unwrap_or(ApprovalReply::Approved);
        Box::pin(async move { reply })
    }
}

/// 승인자 호출 횟수를 세고 정한 응답을 순번대로 돌려주는 가짜 승인자
#[must_use]
pub(in crate::tools) fn counting_approver_with_reply(
    replies: &[ApprovalReply],
) -> (ApproverBox, Arc<AtomicUsize>) {
    let calls = Arc::new(AtomicUsize::new(0));
    let approver = box_of(CountingApprover {
        calls: calls.clone(),
        replies: replies.to_vec(),
    });
    (approver, calls)
}
