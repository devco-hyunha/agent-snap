//! 시간·재시도 설정 값 타입.
//!
//! 빠진 값은 [`HttpSettings::default`]가 채움.

use std::time::Duration;

/// 시간·재시도 값.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpSettings {
    /// 최초 요청 외 재시도 횟수.
    pub retries: u32,
    /// 백오프 시작값.
    pub backoff_min: Duration,
    /// 백오프 상한.
    pub backoff_max: Duration,
    /// `Retry-After`를 따르는 상한.
    pub retry_after_max: Duration,
    /// 연결 단계 제한.
    pub connect_seconds: Duration,
    /// 요청 후 응답 헤더를 기다리는 제한.
    pub header_wait_seconds: Duration,
    /// 스트림에서 다음 조각을 기다리는 제한.
    pub stream_idle_seconds: Duration,
}

impl Default for HttpSettings {
    fn default() -> Self {
        Self {
            retries: 3,
            backoff_min: Duration::from_secs(1),
            backoff_max: Duration::from_secs(30),
            retry_after_max: Duration::from_secs(60),
            connect_seconds: Duration::from_secs(10),
            header_wait_seconds: Duration::from_secs(300),
            stream_idle_seconds: Duration::from_secs(120),
        }
    }
}
