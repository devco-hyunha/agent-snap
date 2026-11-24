//! T2 설정 기본값 시험: 기본값 책임. 빠진 값의 기본값만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn language_defaults_to_ko() {
    let text = "{}";

    let config = config_from_str(text, Path::new(PATH)).expect("test reads empty config");

    assert_eq!(config.language, "ko");
}

#[test]
fn http_defaults_to_seven_values() {
    let text = "{}";

    let config = config_from_str(text, Path::new(PATH)).expect("test reads empty config");

    assert_eq!(config.http.retries, 3);
    assert_eq!(config.http.backoff_min, 1.0);
    assert_eq!(config.http.backoff_max, 30.0);
    assert_eq!(config.http.retry_after_max, 60.0);
    assert_eq!(config.http.connect_seconds, 10.0);
    assert_eq!(config.http.header_wait_seconds, 300.0);
    assert_eq!(config.http.stream_idle_seconds, 120.0);
}

#[test]
fn sampling_keeps_missing_options_none() {
    let text = r#"{ "sampling": { "temperature": 1 } }"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test reads partial sampling");

    let sampling = config.sampling.expect("test has sampling block");
    assert_eq!(sampling.temperature, Some(1.0));
    assert_eq!(sampling.top_p, None);
    assert_eq!(sampling.max_tokens, None);
}
