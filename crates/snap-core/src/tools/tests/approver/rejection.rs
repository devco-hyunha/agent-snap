//! 거부 문구 시험: 승인 거부·만료·미주입 문구와 cancel 문구 구분, 빈 문구 확인

use crate::tools::approver::ApprovalReply;

/// 거부 문구가 공유 앞머리로 시작하는지 확인
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
