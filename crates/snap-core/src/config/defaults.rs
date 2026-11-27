//! snap 설정 기본값 창구 자리. `default_*` 함수 8개만 담음.
//!
//! 기본값 상수는 `consts::defaults`에서 받고, 검증 규칙은 T3가 맡음.

use crate::consts::defaults::DEFAULT_BACKOFF_MAX_SECONDS;
use crate::consts::defaults::DEFAULT_BACKOFF_MIN_SECONDS;
use crate::consts::defaults::DEFAULT_CONNECT_SECONDS;
use crate::consts::defaults::DEFAULT_HEADER_WAIT_SECONDS;
use crate::consts::defaults::DEFAULT_LANGUAGE;
use crate::consts::defaults::DEFAULT_RETRIES;
use crate::consts::defaults::DEFAULT_RETRY_AFTER_MAX_SECONDS;
use crate::consts::defaults::DEFAULT_STREAM_IDLE_SECONDS;

/// 언어 기본값 창구.
pub(crate) fn default_language() -> String {
    DEFAULT_LANGUAGE.to_owned()
}

/// 재시도 횟수 기본값 창구.
pub(crate) const fn default_retries() -> u64 {
    DEFAULT_RETRIES
}

/// 재시도 대기 최소 기본값 창구.
pub(crate) const fn default_backoff_min() -> f64 {
    DEFAULT_BACKOFF_MIN_SECONDS
}

/// 재시도 대기 최대 기본값 창구.
pub(crate) const fn default_backoff_max() -> f64 {
    DEFAULT_BACKOFF_MAX_SECONDS
}

/// 재시도 권유 최대 기본값 창구.
pub(crate) const fn default_retry_after_max() -> f64 {
    DEFAULT_RETRY_AFTER_MAX_SECONDS
}

/// 연결 시도 기본값 창구.
pub(crate) const fn default_connect_seconds() -> f64 {
    DEFAULT_CONNECT_SECONDS
}

/// 헤더 대기 기본값 창구.
pub(crate) const fn default_header_wait_seconds() -> f64 {
    DEFAULT_HEADER_WAIT_SECONDS
}

/// 스트림 대기 기본값 창구.
pub(crate) const fn default_stream_idle_seconds() -> f64 {
    DEFAULT_STREAM_IDLE_SECONDS
}
