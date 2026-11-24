//! T2 설정 읽기 시험: 값 책임. 샘플링·http 값 읽기만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn reads_sampling_and_http_values() {
    let text = r#"{
  "sampling": { "temperature": 0.5, "top_p": 0.9, "max_tokens": 1200 },
  "http": { "retries": 5, "backoff_min": 2, "backoff_max": 20, "retry_after_max": 45,
            "connect_seconds": 8, "header_wait_seconds": 200, "stream_idle_seconds": 90 }
}"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test reads sampling and http");

    let sampling = config.sampling.expect("test has sampling block");
    assert_eq!(sampling.temperature, Some(0.5));
    assert_eq!(sampling.top_p, Some(0.9));
    assert_eq!(sampling.max_tokens, Some(1200));
    assert_eq!(config.http.retries, 5);
    assert_eq!(config.http.backoff_min, 2.0);
    assert_eq!(config.http.backoff_max, 20.0);
    assert_eq!(config.http.retry_after_max, 45.0);
    assert_eq!(config.http.connect_seconds, 8.0);
    assert_eq!(config.http.header_wait_seconds, 200.0);
    assert_eq!(config.http.stream_idle_seconds, 90.0);
}
