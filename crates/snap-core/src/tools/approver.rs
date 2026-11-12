//! 승인자(CommandApprover) 트레이트와 승인 대기 한 자리만 둡니다(03-6 T1, spec AC 1·AC 2, SRP).
//!
//! CLI가 승인자를 주입하고(D22 5.3절), 도구는 이 트레이트 한 동작만 부릅니다.
//! 승인 대기 상한 기본값(300초)은 `config`의 `agent` 섹션 기본값 상수를 그대로 이어받습니다.
//! 만료는 거부로만 봅니다(D22: L3 무인 아님). 승인 대상은 `run_command` 하나이며
//! 파일 쓰기는 대상이 아닙니다(D49).

use tokio_util::sync::CancellationToken;

use super::run_command_settings::RunCommandSettings;

/// 승인자가 받은 응답을 나타내는 값입니다.
///
/// 승인자 미주입·비대화형은 `Missing`으로 기다리지 않고 즉시 거부하고, 상한 안의 응답만
/// 승인·거부로 봅니다(spec AC 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalReply {
    /// 사용자가 승인했습니다.
    Approved,
    /// 사용자가 거부했습니다.
    Rejected,
    /// 승인 응답 상한 안에 응답이 없습니다. D22대로 거부로만 봅니다.
    Expired,
    /// 승인자 미주입·비대화형입니다. 기다리지 않고 즉시 거부합니다.
    Missing,
    /// 승인 대기 중 cancel로 끝난 경우입니다. 사용자가 끊지 않은 자리니 결과 표기를 `중단`으로 가립니다.
    Cancelled,
}

impl ApprovalReply {
    /// 명령을 실행해도 되는지 여부입니다.
    #[must_use]
    pub const fn is_approved(self) -> bool {
        matches!(self, Self::Approved)
    }

    /// 모델에 보내는 도구 결과 문구를 만드는 한 자리입니다. 거부·만료·미주입은 `오류: 승인 거부`로,
    /// cancel로 끊긴 대기은 `오류: 중단`으로 시작하게 둡니다.
    /// 03-2 골격의 즉시 거부 문구와 같은 형태를 유지해 시험과 로그가 이어지게 합니다(DRY: 문구는 이 한 자리, dev-principles 3절).
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

/// 승인자 트레이트 객체를 담는 창구입니다. `ToolContext.approver`의 타입 정본입니다(03-1 골격 자리).
pub type ApproverBox = Box<dyn CommandApprover + Send + Sync>;

/// 승인자가 돌려주는 미래 타입입니다. `ToolCallFuture`과 같은 핀 상자 규칙을 따릅니다(DRY).
pub type ApprovalFuture<'a> =
    std::pin::Pin<Box<dyn std::future::Future<Output = ApprovalReply> + Send + 'a>>;

/// 명령 실행 전에 사람 확인을 받는 승인자입니다(D22 5.3절: CLI가 주입, L1은 명령마다 확인).
pub trait CommandApprover: Send + Sync {
    /// 한 줄 요약을 받아 승인·거부를 돌려줍니다.
    fn approve<'a>(&'a self, summary: &'a str) -> ApprovalFuture<'a>;
}

/// 승인자를 호출하고 승인 대기 상한과 cancel로 감싼 한 자리입니다(spec AC 1·AC 2·AC 4).
///
/// 승인자가 없으면 기다리지 않고 즉시 거부하고, 상한 안의 응답만 승인·거부로 봅니다.
/// cancel은 응답을 기다리는 동안에도 듣습니다(D22: 무한 대기 금지).
/// cancel로 끝난 대기거 거부로 섞이지 않게 `Cancelled`로 따로 돌려줍니다(spec 출력 표의 `중단`).
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
