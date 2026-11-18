//! 승인자 트레이트와 승인 대기 한 자리
//!
//! 만료는 거부로만 봄

use tokio_util::sync::CancellationToken;

use crate::tools::run_command_settings::RunCommandSettings;

/// 승인자가 받은 응답을 나타내는 값
///
/// 승인자 미주입·비대화형은 `Missing`으로 즉시 거부. 상한 안의 응답만 승인·거부로 봄
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalReply {
    /// 사용자가 승인함
    Approved,
    /// 사용자가 거부함
    Rejected,
    /// 승인 응답 상한 안에 응답이 없음. 거부로만 봄
    Expired,
    /// 승인자 미주입·비대화형. 기다리지 않고 즉시 거부
    Missing,
    /// 승인 대기 중 cancel로 끝난 경우. 사용자가 끊지 않은 자리니 결과 표기를 `중단`으로 가림
    Cancelled,
}

impl ApprovalReply {
    /// 명령을 실행해도 되는지 여부
    #[must_use]
    pub const fn is_approved(self) -> bool {
        matches!(self, Self::Approved)
    }

    /// 도구 결과 문구 생성. 거부·만료·미주입은 `오류: 승인 거부`, cancel은 `오류: 중단`으로 시작
    #[must_use]
    pub fn rejection_text(self, summary: &str) -> String {
        let base = match self {
            Self::Approved => return String::new(),
            Self::Rejected => "오류: 승인 거부",
            Self::Expired => "오류: 승인 거부(응답 없음)",
            Self::Missing => "오류: 승인 거부(승인자 없음)",
            Self::Cancelled => "오류: 중단",
        };
        format!("{base} {summary}")
    }
}

/// 승인자 트레이트 객체를 담는 창구. `ToolContext.approver`의 타입 정본
///
/// JS 엔진 스레드가 승인자를 실행 판 복제본과 함께 쓰므로 공유 창구로 둠
pub type ApproverBox = std::sync::Arc<dyn CommandApprover + Send + Sync>;

/// 승인자가 돌려주는 미래 타입. `ToolCallFuture`과 같은 핀 상자 규칙
pub type ApprovalFuture<'a> =
    std::pin::Pin<Box<dyn std::future::Future<Output = ApprovalReply> + Send + 'a>>;

/// 명령 실행 전에 사람 확인을 받는 승인자(CLI가 주입)
pub trait CommandApprover: Send + Sync {
    /// 한 줄 요약을 받아 승인·거부를 반환
    fn approve<'a>(&'a self, summary: &'a str) -> ApprovalFuture<'a>;
}

/// 승인자를 호출하고 승인 대기 상한과 cancel로 감싼 한 자리
///
/// 승인자가 없으면 기다리지 않고 즉시 거부. cancel은 응답을 기다리는 동안에도 듣음
#[must_use]
pub async fn request_approval(
    approver: Option<&ApproverBox>,
    summary: &str,
    settings: &RunCommandSettings,
    cancel: &CancellationToken,
) -> ApprovalReply {
    let Some(approver) = approver else {
        return ApprovalReply::Missing;
    };
    tokio::select! {
        () = cancel.cancelled() => ApprovalReply::Cancelled,
        _ = tokio::time::sleep(settings.approval_limit) => ApprovalReply::Expired,
        reply = approver.approve(summary) => reply,
    }
}
