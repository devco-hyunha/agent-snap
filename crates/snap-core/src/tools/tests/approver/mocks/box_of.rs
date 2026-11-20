//! 승인자를 공유 창구 하나로 감싸는 한 자리
//!
//! 같은 계통 가짜 승인자 파일들이 재사용

use std::sync::Arc;

use crate::tools::approval::request::ApproverBox;
use crate::tools::approver::CommandApprover;

/// 승인자를 공유 창구 하나로 감싸는 한 자리
#[must_use]
pub(super) fn box_of(approver: impl CommandApprover + 'static) -> ApproverBox {
    Arc::new(approver)
}
