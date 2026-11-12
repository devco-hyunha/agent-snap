//! 크레이트 어디서나 쓰는 '지금 시각' 계산을 한곳에 두는 자리입니다(03-6 R1 감사 분리).
//!
//! 도구 호출 기록 채움(`agent/record_fill.rs`)과 2안 기록(`agent/text/record.rs`)이 같은 기준을
//! 쓰므로, 시각 함수를 크레이트 공용 파일로 옮겨 두 곳에서 함께 씁니다(dev-principles 3절 DRY).

use std::time::{SystemTime, UNIX_EPOCH};

/// 지금 시각을 유닉스 초로 바꿉니다. 시험은 하한·상한으로만 비교합니다.
///
/// 2안 기록(text_record)도 같은 기준을 쓰게 `pub(crate)`로 열어 두 곳에서 함께 씁니다(DRY).
pub(crate) fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}
