//! 시험용 승인 대기 상한 재료: 승인 대기 상한만 짧게 지정한 설정
//!
//! 승인 대기 상한은 승인자 응답 방식과 따로 바꾸는 값이라 실행 상한과 분리

use std::time::Duration;

use crate::config::AgentSettings;
use crate::tools::approval::settings::ApprovalSettings;

/// 상한을 짧게 지정한 승인 대기 상한(초). 승인자가 즉시 답하므로 시험은 짧게 씀
#[must_use]
pub fn approval_limit(secs: u64) -> ApprovalSettings {
    ApprovalSettings::from_settings(&AgentSettings {
        approval_seconds: Duration::from_secs(secs),
        ..AgentSettings::default()
    })
}
