//! URL·시크릿 가림·길이 제한 단위 테스트입니다.

use crate::llm::error::redact::summarize_stream_error;

#[test]
fn summarize_redacts_url_and_secret_like_token() {
    let secret = "sk-leak-key-value-1234567890abcd";
    let host = "https://evil.example/v1/chat";
    let raw = format!("fail {secret} at {host}");
    let summary = summarize_stream_error(&raw);
    assert!(!summary.contains(secret));
    assert!(!summary.contains("evil.example"));
    assert!(!summary.contains(host));
    assert!(summary.contains("[가림]"));
    assert!(summary.contains("[주소]"));
}

#[test]
fn summarize_truncates_long_text() {
    let raw = "가".repeat(250);
    let summary = summarize_stream_error(&raw);
    assert!(summary.chars().count() <= 201);
    assert!(summary.ends_with('…'));
}
