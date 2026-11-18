//! execute 브리지: 호출마다 승인 대기를 여는 창구.
//!
//! 호출 전체 확인 승인이 끝난 뒤에도 쓰기·명령 도구는 호출마다 승인을 다시 받음.
//! 엔진 스레드가 오래 쓰므로 실행 판 복제본을 소유로 가짐(빌린 참조의 수명 연장 금지).

use std::sync::Arc;

use crate::tools::approval::approver::ApprovalReply;
use crate::tools::approval::approver::request_approval;
use crate::tools::tool::ToolContext;

/// 호출 전용 승인 대기 창구.
///
/// 호출마다 `request_approval`을 부르고 그 상한·cancel을 그대로 씀.
pub(in crate::tools) struct ScriptApproval {
    context: Arc<ToolContext>,
}

impl ScriptApproval {
    /// 호출마다 쓸 승인 창구를 실행 판 복제본과 함께 만듦.
    pub(in crate::tools) const fn new(context: Arc<ToolContext>) -> Self {
        Self { context }
    }

    /// 쓰기·명령 호출마다 승인 대기를 열고 승인·거부 결과를 승인 응답으로 반환.
    ///
    /// 거부 문구는 `request_approval` 정본을 재사용해 승인 로그와 통일.
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
