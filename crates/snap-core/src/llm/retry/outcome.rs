//! `Retry-After` 해석 결과 타입 자리.

use std::time::Duration;

/// `Retry-After` 헤더를 해석하고 상한과 비교한 결과.
///
/// 포기는 대기 초만 담고 키·주소·응답 본문은 담지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryAfterOutcome {
    /// 상한 이하이므로 이 시간만큼 기다림. 0초 포함.
    Wait(Duration),

    /// 헤더가 없거나 음수·해석 불가. 백오프 경로로 감.
    Absent,

    /// 상한을 넘는 대기라 대기 초를 밝히고 포기.
    ExceedsCap {
        /// 헤더가 지시한 대기 시간.
        wait: Duration,
    },
}
