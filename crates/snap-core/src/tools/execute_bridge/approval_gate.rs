//! 03-8 execute 브리지: 호출마다 승인 대기를 여는 창구만 둡니다(AC 2·AC 5).
//!
//! 결론: `execute` 호출 전체 확인 1승인이 끝난 뒤에도, 그 안의 쓰기·명령 도구는 호출마다
//! 승인을 다시 타야 합니다(D22 5.3절: 명령마다 확인). 이 파일은 승인 대기 창구만 맡고,
//! 기록 줄은 `script_log.rs`, 요약 글자는 `text_parts.rs`가 맡습니다(dev-principles 2절 SRP).
//! 03-6 `request_approval`과 03-6 정본 문구를 그대로 재사용합니다.
//!
//! 03-8 T2 lifetime 보정: 엔진 스레드가 승인 창구를 `'static`보다 오래 쓰므로, 실행 판 복제본을
//! 소유로 받아 빌린 수명을 두지 않습니다(빌린 참조가 `'static`보다 오래 사는 구조 금지).

use std::sync::Arc;

use crate::tools::approval::approver::ApprovalReply;
use crate::tools::approval::approver::request_approval;
use crate::tools::tool::ToolContext;

/// 03-8 호출 전용 승인 대기 창구입니다.
///
/// 호출마다 03-6 `request_approval`을 부르고, 03-6의 승인 대기 상한·cancel을 그대로 씁니다.
/// 엔진 스레드가 승인 창구를 오래 쓰므로, 빌린 판이 아니라 복제본을 소유로 가집니다.
pub(in crate::tools) struct ScriptApproval {
    context: Arc<ToolContext>,
}

impl ScriptApproval {
    /// 호출마다 쓸 승인 창구를 실행 판 복제본과 함께 만듭니다.
    pub(in crate::tools) const fn new(context: Arc<ToolContext>) -> Self {
        Self { context }
    }

    /// 쓰기·명령 호출마다 승인 대기를 열고, 승인·거부 결과를 승인 응답으로 돌려줍니다.
    ///
    /// 거부는 03-6 정본 문구(`오류: 승인 거부 …`)를 그대로 쓰므로, 승인 로그 문구가
    /// 같은 도구의 1안 경로와 같아집니다(dev-principles 3절 DRY).
    pub(in crate::tools) async fn gate(&self, summary: &str) -> ApprovalReply {
        request_approval(
            self.context.approver.as_ref(),
            summary,
            &self.context.run_command,
            &self.context.cancel_token(),
        )
        .await
    }
}
