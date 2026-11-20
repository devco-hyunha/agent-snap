//! 셸 시험의 승인자·시간 상한 재료 정본
//!
//! 승인자는 즉시 답하므로 승인 대기 상한은 짧게 고정

use crate::tools::tool::ToolContext;

use super::approver::mocks::approved_approver;
use super::support::{TempFolder, approval_limit, context_of, run_settings};

/// 시험이 승인 대기에 쓰는 상한 초. 승인자가 즉시 답하므로 짧게 고정
pub const APPROVAL_TEST_SECONDS: u64 = 5;

/// 승인자와 시간 상한을 채운 작업 폴더 재료 생성. 셸 시험이 같은 조립을 다시 적지 않게 씀
#[must_use]
pub fn approved_run_context(folder: &TempFolder, run_seconds: u64) -> ToolContext {
    let mut ctx = context_of(folder);
    ctx.run_command = run_settings(run_seconds);
    ctx.approval = approval_limit(APPROVAL_TEST_SECONDS);
    ctx.approver = Some(approved_approver());
    ctx
}
