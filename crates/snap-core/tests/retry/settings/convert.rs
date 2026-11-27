//! 02-2 `http` 값 → 02-4 판단 설정 변환 시험. 기본값 일치와 초→시간 옮기기만 봄.

use std::time::Duration;

use snap_core::config::HttpSettings;
use snap_core::llm::retry_settings_of;

/// 02-2 기본값과 같은 설정. 재시도 3회, 지수 대기 1~30초, `Retry-After` 상한 60초.
fn default_settings() -> HttpSettings {
    HttpSettings::default()
}

/// 기본 `http` 값의 판단 설정이 02-2 기본값과 같음.
#[test]
fn default_http_values_match_retry_defaults() {
    let settings = retry_settings_of(&default_settings());

    assert_eq!(
        settings,
        snap_core::llm::RetrySettings {
            max_retries: 3,
            backoff_min: Duration::from_secs(1),
            backoff_max: Duration::from_secs(30),
            retry_after_max: 60,
        },
        "default http values convert to the 02-4 defaults"
    );
}

/// 초 값과 정수 값을 그대로 옮김.
#[test]
fn converts_seconds_and_integer_fields() {
    let http = HttpSettings {
        retries: 5,
        backoff_min: 0.25,
        backoff_max: 12.5,
        retry_after_max: 42.0,
        ..default_settings()
    };

    let settings = retry_settings_of(&http);

    assert_eq!(
        settings,
        snap_core::llm::RetrySettings {
            max_retries: 5,
            backoff_min: Duration::from_millis(250),
            backoff_max: Duration::from_millis(12_500),
            retry_after_max: 42,
        },
        "seconds and integer fields move as given"
    );
}

/// `Retry-After` 상한 초값은 소수점 아래를 반올림해 정수로 옮김.
#[test]
fn retry_after_max_seconds_becomes_integer() {
    let http = HttpSettings {
        retry_after_max: 9.6,
        ..default_settings()
    };

    assert_eq!(
        retry_settings_of(&http).retry_after_max,
        10,
        "fractional seconds round to the nearest integer"
    );
}
