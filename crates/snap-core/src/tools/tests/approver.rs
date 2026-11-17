//! 승인자 호출 시험(03-6 spec AC 1·AC 2·AC 4): 승인·거부·만료·미주입과 cancel 연결을 확인한다.
//!
//! 승인자는 가짜(mock)로 만들고, 시간 상한은 짧게 주며, 경과 시간은 하한·상한으로만 본다(테스트 계획).

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::config::AgentSettings;
use crate::tools::approver::{
    ApprovalFuture, ApprovalReply, ApproverBox, CommandApprover, request_approval,
};
use crate::tools::run_command_settings::RunCommandSettings;

use super::support::TempFolder;

/// 즉시 돌려주는 가짜 승인자입니다. 응답을 `ApprovalReply`로 고정해 시험이 같은 조립을 다시 쓰지 않게 합니다.
struct MockApprover {
    reply: ApprovalReply,
}

impl CommandApprover for MockApprover {
    fn approve<'a>(&'a self, _summary: &'a str) -> ApprovalFuture<'a> {
        Box::pin(async move { self.reply })
    }
}

/// 응답을 기다리는 가짜 승인자입니다. 상한이 먼저 오는지 보는 만료 시험에 씁니다.
struct WaitingApprover;

impl CommandApprover for WaitingApprover {
    fn approve<'a>(&'a self, _summary: &'a str) -> ApprovalFuture<'a> {
        Box::pin(async move { std::future::pending().await })
    }
}

fn box_of(approver: impl CommandApprover + 'static) -> ApproverBox {
    Arc::new(approver)
}

/// 즉시 승인을 돌려주는 가짜 승인자 창구입니다(03-8 execute 시험에서 재사용).
#[must_use]
pub(in crate::tools) fn approved_approver() -> ApproverBox {
    box_of(MockApprover {
        reply: ApprovalReply::Approved,
    })
}

/// 승인자를 부른 횟수를 세고, 정한 응답을 순번대로 돌려주는 가짜 승인자입니다(03-8 AC 2·AC 5 확인).
///
/// 통합 시험 트리의 `counting_approver`와 같은 성질이지만 두 compile 단위(단위 시험·통합 시험)가
/// 서로를 볼 수 없어, 이 트리에 한 번만 두고 필요한 시험이 다 씁니다(재사용, 한 정의).
#[must_use]
pub(in crate::tools) fn counting_approver_with_reply(
    replies: &[ApprovalReply],
) -> (ApproverBox, Arc<AtomicUsize>) {
    let calls = Arc::new(AtomicUsize::new(0));
    let approver = Arc::new(CountingApprover {
        calls: calls.clone(),
        replies: replies.to_vec(),
    });
    (approver, calls)
}

/// 응답을 정해진 순번대로 돌려주고 불린 횟수를 세는 가짜 승인자입니다.
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

/// 상한을 짧게 주입한 설정입니다(승인 대기 초, 실행 상한은 60초 — 시험 계획대로 값 주입).
fn approval_settings(secs: u64) -> RunCommandSettings {
    RunCommandSettings::from_settings(&AgentSettings {
        run_command_seconds: Duration::from_secs(60),
        approval_seconds: Duration::from_secs(secs),
    })
}

/// AC 1: 승인자 미주입은 기다리지 않고 즉시 거부로 봅니다.
#[tokio::test]
async fn missing_approver_is_immediately_rejected() {
    let folder = TempFolder::new("approval-missing");
    let ctx = super::support::context_of(&folder);
    let reply = request_approval(
        None,
        "echo snap",
        &approval_settings(5),
        &ctx.cancel_token(),
    )
    .await;
    assert_eq!(reply, ApprovalReply::Missing);
    assert!(!reply.is_approved());
}

/// AC 1: 승인자 mock의 승인은 실행 허용으로 이어집니다.
#[tokio::test]
async fn approved_reply_is_accepted() {
    let ctx = super::support::context_of(&TempFolder::new("approval-ok"));
    let approver = box_of(MockApprover {
        reply: ApprovalReply::Approved,
    });
    let reply = request_approval(
        Some(&approver),
        "echo snap",
        &approval_settings(5),
        &ctx.cancel_token(),
    )
    .await;
    assert!(reply.is_approved());
}

/// AC 1: 승인자 mock의 거부는 그대로 거부로 봅니다.
#[tokio::test]
async fn rejected_reply_is_rejected() {
    let ctx = super::support::context_of(&TempFolder::new("approval-reject"));
    let approver = box_of(MockApprover {
        reply: ApprovalReply::Rejected,
    });
    let reply = request_approval(
        Some(&approver),
        "echo snap",
        &approval_settings(5),
        &ctx.cancel_token(),
    )
    .await;
    assert_eq!(reply, ApprovalReply::Rejected);
}

/// AC 2: 응답이 상한 안에 없으면 만료로 보고, 그 시간 안에 돌아옵니다(넉넉한 상한만 확인).
#[tokio::test]
async fn approval_reply_expires_within_upper_bound() {
    let ctx = super::support::context_of(&TempFolder::new("approval-expire"));
    let approver = box_of(WaitingApprover);
    let started = std::time::Instant::now();
    let reply = tokio::time::timeout(
        Duration::from_secs(6),
        request_approval(
            Some(&approver),
            "echo snap",
            &approval_settings(1),
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

/// AC 4: 승인 대기 중 cancel은 사용자 거부와 구분해 `Cancelled`로 돌려준다.
#[tokio::test]
async fn cancel_during_approval_wait_is_cancelled() {
    let ctx = super::support::context_of(&TempFolder::new("approval-cancel"));
    let token = ctx.cancel_token();
    token.cancel();
    let approver = box_of(WaitingApprover);
    let reply =
        request_approval(Some(&approver), "echo snap", &approval_settings(30), &token).await;
    assert_eq!(reply, ApprovalReply::Cancelled);
    assert!(!reply.is_approved());
}

/// 거부·만료·미주입은 `오류: 승인 거부`로 시작하고, cancel로 끊긴 대기은 `오류: 중단`으로 시작합니다
/// (문구 정본 한 자리). 요약은 한 줄로 뒤에 붙습니다.
#[test]
fn rejection_phrase_starts_with_the_shared_prefix() {
    for reply in [
        ApprovalReply::Rejected,
        ApprovalReply::Expired,
        ApprovalReply::Missing,
    ] {
        let text = reply.rejection_text("echo snap");
        assert!(text.starts_with("오류: 승인 거부"), "{text}");
        assert_eq!(text.lines().count(), 1, "한 줄만: {text}");
        assert!(text.contains("echo snap"));
    }
    let cancelled = ApprovalReply::Cancelled.rejection_text("echo snap");
    assert!(
        cancelled.starts_with("오류: 중단"),
        "cancel 표기 구분: {cancelled}"
    );
    assert_eq!(cancelled.lines().count(), 1, "한 줄만: {cancelled}");
    assert!(
        ApprovalReply::Approved
            .rejection_text("echo snap")
            .is_empty()
    );
}
