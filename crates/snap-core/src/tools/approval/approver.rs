//! 승인 응답 값과 승인자 트레이트만 두는 자리
//!
//! 만료는 거부로만 봄. 거절 문구는 `rejection_text`에서 만든다.

use super::request::ApprovalFuture;

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
}

/// 명령 실행 전에 사람 확인을 받는 승인자(CLI가 주입)
pub trait CommandApprover: Send + Sync {
    /// 한 줄 요약을 받아 승인·거부를 반환
    fn approve<'a>(&'a self, summary: &'a str) -> ApprovalFuture<'a>;
}
