//! 02-4 재시도 시험 재료 정본. 설정·기준 시각·난수 공급원을 한곳에 모음.

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use snap_core::llm::RandomSource;
use snap_core::llm::RetrySettings;

/// 02-2 `http` 기본값과 같은 설정. 재시도 3회, 지수 대기 1~30초, `Retry-After` 상한 60초.
pub(crate) const SETTINGS: RetrySettings = RetrySettings {
    max_retries: 3,
    backoff_min: Duration::from_secs(1),
    backoff_max: Duration::from_secs(30),
    retry_after_max: 60,
};

/// 기준 시각. 1970-01-01 00:00:00 UTC.
pub(crate) const NOW: SystemTime = UNIX_EPOCH;

/// 지정 값 하나를 뽑는 공급원. 뽑은 값이 그대로 대기가 되는지 확인용.
pub(crate) struct FixedSource {
    /// 뽑을 값.
    pub(crate) value: u64,
}

impl RandomSource for FixedSource {
    fn next_inclusive(&mut self, _max: u64) -> u64 {
        self.value
    }
}

/// 항상 0을 뽑는 공급원. 대기 0 하한 확인용.
pub(crate) struct ZeroSource;

impl RandomSource for ZeroSource {
    fn next_inclusive(&mut self, _max: u64) -> u64 {
        0
    }
}

/// 넘어온 상한을 뽑는 공급원. 대기 상한 경계 확인용.
pub(crate) struct MaxSource;

impl RandomSource for MaxSource {
    fn next_inclusive(&mut self, max: u64) -> u64 {
        max
    }
}
