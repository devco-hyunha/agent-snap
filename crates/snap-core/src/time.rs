//! 크레이트 어디서나 쓰는 '지금 시각' 계산을 한곳에 두는 자리.

use std::time::{SystemTime, UNIX_EPOCH};

/// 지금 시각을 유닉스 초로 변환. 시험은 하한·상한으로만 비교.
pub(crate) fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}
