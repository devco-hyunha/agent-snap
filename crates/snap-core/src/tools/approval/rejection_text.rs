//! 승인 거부·만료·미주입·cancel 도구 결과 문구 생성 한 자리

use super::approver::ApprovalReply;

impl ApprovalReply {
    /// 도구 결과 문구 생성. 거부·만료·미주입은 `오류: 승인 거부`, cancel은 `오류: 중단`으로 시작
    ///
    /// 문구는 한 줄만 남기고 요약을 뒤에 붙임
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
