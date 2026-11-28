//! T3 검증 시험: backoff 범위 책임. 상한 역전 거부와 같은 값 허용만 봄.
use crate::material::parse;
use crate::material::reject_message;

#[test]
fn rejects_backoff_min_above_max() {
    let text = r#"{ "http": { "backoff_min": 30, "backoff_max": 1 } }"#;

    let message = reject_message(text);

    assert!(
        message.contains("backoff_min") && message.contains("backoff_max"),
        "message names both items: {message}"
    );
    assert!(
        !message.contains("30"),
        "message hides input value: {message}"
    );
}

#[test]
fn accepts_equal_backoff_values() {
    let text = r#"{ "http": { "backoff_min": 5, "backoff_max": 5 } }"#;

    let config = parse(text).expect("test accepts equal backoff values");

    assert_eq!(config.http.backoff_min, 5.0);
}
