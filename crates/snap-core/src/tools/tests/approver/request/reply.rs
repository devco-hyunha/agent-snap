//! 승인 요청 결과 시험: 미주입·승인·거부 응답을 확인

use crate::tools::approval::request::request_approval;
use crate::tools::approver::ApprovalReply;

use super::super::super::support::{TempFolder, approval_limit, context_of};
use super::super::mocks::{approved_approver, rejected_approver};

/// 승인자 미주입은 기다리지 않고 즉시 거부로 봄
#[tokio::test]
async fn missing_approver_is_immediately_rejected() {
    let folder = TempFolder::new("approval-missing");
    let ctx = context_of(&folder);
    let reply = request_approval(None, "echo snap", &approval_limit(5), &ctx.cancel_token()).await;
    assert_eq!(reply, ApprovalReply::Missing);
    assert!(!reply.is_approved());
}

/// 승인자 mock의 승인은 실행 허용으로 이어짐
#[tokio::test]
async fn approved_reply_is_accepted() {
    let ctx = context_of(&TempFolder::new("approval-ok"));
    let approver = approved_approver();
    let reply = request_approval(
        Some(&approver),
        "echo snap",
        &approval_limit(5),
        &ctx.cancel_token(),
    )
    .await;
    assert!(reply.is_approved());
}

/// 승인자 mock의 거부는 그대로 거부로 봄
#[tokio::test]
async fn rejected_reply_is_rejected() {
    let ctx = context_of(&TempFolder::new("approval-reject"));
    let approver = rejected_approver();
    let reply = request_approval(
        Some(&approver),
        "echo snap",
        &approval_limit(5),
        &ctx.cancel_token(),
    )
    .await;
    assert_eq!(reply, ApprovalReply::Rejected);
}
