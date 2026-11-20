//! 승인 요청 시간 시험: 상한 만료와 cancel 연결을 확인
//!
//! 경과는 넉넉한 하한·상한으로만 봄

use std::time::Duration;

use crate::tools::approval::request::request_approval;
use crate::tools::approver::ApprovalReply;

use super::super::super::support::{TempFolder, approval_limit, context_of};
use super::super::mocks::waiting_approver_box;

/// 응답이 상한 안에 없으면 만료로 보고, 넉넉한 상한 안에 돌아옴
#[tokio::test]
async fn approval_reply_expires_within_upper_bound() {
    let ctx = context_of(&TempFolder::new("approval-expire"));
    let approver = waiting_approver_box();
    let started = std::time::Instant::now();
    let reply = tokio::time::timeout(
        Duration::from_secs(6),
        request_approval(
            Some(&approver),
            "echo snap",
            &approval_limit(1),
            &ctx.cancel_token(),
        ),
    )
    .await
    .expect("만료는 6초 안에 돌아와야 합니다");
    assert_eq!(reply, ApprovalReply::Expired);
    assert!(
        started.elapsed() >= Duration::from_millis(900),
        "상한 전에 만료되면 안 됩니다"
    );
}

/// 승인 대기 중 cancel은 사용자 거부와 구분해 `Cancelled`로 돌려줌
#[tokio::test]
async fn cancel_during_approval_wait_is_cancelled() {
    let ctx = context_of(&TempFolder::new("approval-cancel"));
    let token = ctx.cancel_token();
    token.cancel();
    let approver = waiting_approver_box();
    let reply = request_approval(Some(&approver), "echo snap", &approval_limit(30), &token).await;
    assert_eq!(reply, ApprovalReply::Cancelled);
    assert!(!reply.is_approved());
}
