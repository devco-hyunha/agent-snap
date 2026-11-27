//! 02-2 `http` 값 → 02-4 판단 설정 변환 자리. f64 초를 `Duration`·정수로 옮기기만 함.

use std::time::Duration;

use crate::config::types::HttpSettings;
use crate::llm::retry::RetrySettings;

/// 초 초값 상한 경계. 1e10초는 나초로 옮겨 `u64` 안에 든다. 이 이상은 시간 값·정수 값의 최대로 봄.
const MAX_SECONDS_BOUND: f64 = 1e10;

/// 설정 `http` 값에서 재시도 판단 설정만 옮김.
///
/// 02-4 판단은 이 설정만 받고 `HttpSettings`는 받지 않음.
pub fn retry_settings_of(settings: &HttpSettings) -> RetrySettings {
    RetrySettings {
        max_retries: settings.retries,
        backoff_min: seconds_duration(settings.backoff_min),
        backoff_max: seconds_duration(settings.backoff_max),
        retry_after_max: seconds_integer(settings.retry_after_max),
    }
}

/// 초 초값을 `Duration`으로 옮김. 0 미만은 0으로 봄.
///
/// 검증이 통과한 값만 받음. 범위를 넘긴 값은 `Duration`의 최대로 봄.
fn seconds_duration(seconds: f64) -> Duration {
    if !seconds.is_finite() || seconds >= MAX_SECONDS_BOUND {
        return Duration::MAX;
    }
    Duration::from_secs_f64(seconds.max(0.0))
}

/// 초 초값을 정수로 옮김. 0 미만은 0으로 봄.
fn seconds_integer(seconds: f64) -> u64 {
    let rounded = seconds.max(0.0).round();
    if rounded >= MAX_SECONDS_BOUND {
        return u64::MAX;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let value = rounded as u64;
    value
}
