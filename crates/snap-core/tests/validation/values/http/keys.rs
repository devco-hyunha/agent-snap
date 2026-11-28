//! T3 검증 시험: http 키·값 책임. 모르는 키 거부와 값 상한 거부만 봄.
use crate::material::reject_message;

#[test]
fn rejects_unknown_http_key() {
    let text = r#"{ "http": { "timeout": 5 } }"#;

    let message = reject_message(text);

    assert!(
        message.contains("http.timeout"),
        "message names http item: {message}"
    );
    assert!(
        !message.contains("5"),
        "message hides input value: {message}"
    );
}

#[test]
fn rejects_bad_http_values() {
    let cases = [
        (
            r#"{ "http": { "connect_seconds": 0 } }"#,
            "http.connect_seconds",
        ),
        (
            r#"{ "http": { "connect_seconds": -1 } }"#,
            "http.connect_seconds",
        ),
        (r#"{ "http": { "retries": -2 } }"#, "http.retries"),
        (r#"{ "http": { "retries": 1.5 } }"#, "http.retries"),
        (
            r#"{ "http": { "backoff_min": "text" } }"#,
            "http.backoff_min",
        ),
    ];
    for (text, item) in cases {
        let message = reject_message(text);

        assert!(
            message.contains(item),
            "message names {item} in {text}: {message}"
        );
    }
}
