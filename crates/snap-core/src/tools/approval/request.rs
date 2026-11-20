//! 승인자 창구 타입과 승인 대기 한 자리
//!
//! 만료는 거부로만 봄

use tokio_util::sync::CancellationToken;

use super::approver::{ApprovalReply, CommandApprover};
use super::settings::ApprovalSettings;

/// 승인자 트레이트 객체를 담는 창구. `ToolContext.approver`의 타입 정본
///
/// JS 엔진 스레드가 승인자를 실행 판 복제본과 함께 쓰므로 공유 창구로 둠
pub type ApproverBox = std::sync::Arc<dyn CommandApprover + Send + Sync>;

/// 승인자가 돌려주는 미래 타입. `ToolCallFuture`과 같은 핀 상자 규칙
pub type ApprovalFuture<'a> =
    std::pin::Pin<Box<dyn std::future::Future<Output = ApprovalReply> + Send + 'a>>;

/// 승인자를 호출하고 승인 대기 상한과 cancel로 감싼 한 자리
///
/// 승인자가 없으면 기다리지 않고 즉시 거부. cancel은 응답을 기다리는 동안에도 듣음
#[must_use]
pub async fn request_approval(
    approver: Option<&ApproverBox>,
    summary: &str,
    settings: &ApprovalSettings,
    cancel: &CancellationToken,
) -> ApprovalReply {
    let Some(approver) = approver else {
        return ApprovalReply::Missing;
    };
    tokio::select! {
        () = cancel.cancelled() => ApprovalReply::Cancelled,
        _ = tokio::time::sleep(settings.limit) => ApprovalReply::Expired,
        reply = approver.approve(summary) => reply,
    }
}
