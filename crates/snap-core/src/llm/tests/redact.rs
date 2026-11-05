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

// AC 2(D28)·AC 6: 서버 오류 본문에서 키·호스트를 가린 뒤에도 길이를 제한한다.
#[test]
fn rough_summary_is_length_limited() {
    let key = "sk-leak-key-value-1234567890abcd";
    let body = format!("{}{}", "자".repeat(400), key);
    let summary = summarize_stream_error(&body);
    let count = summary.chars().count();
    assert!(count <= 201, "요약 길이 {count}");
    assert!(!summary.contains(key));
}

// AC 2(D28): 어림 규칙을 통과한 문구도, 이 요청에서 알고 있는 키·base_url이면 한 번 더 가린다.
#[test]
fn scrub_hides_known_key_and_host_after_rough_pass() {
    use crate::llm::error::scrub::SecretScrub;
    use crate::secrets::SecretValue;

    // 키가 어림 규칙의 짧은 토큰 기준에 안 걸리고(16자 미만), host는 scheme이 없어
    // 어림 가리기가 그대로 남기는 자리입니다. 확인 가리기가 한 번 더 지워야 합니다.
    let key = SecretValue::new("sk-known-key");
    let echo = "upstream key sk-known-key at 127.0.0.1:6099 /v1 failed";
    let rough = summarize_stream_error(echo);
    assert!(
        rough.contains("sk-known-key"),
        "어림만으로는 키가 남아야 합니다"
    );
    assert!(
        rough.contains("127.0.0.1"),
        "어림만으로는 호스트가 남아야 합니다"
    );

    let scrub = SecretScrub::new(Some(&key), &SecretValue::new("http://127.0.0.1:6099/v1"));
    let cleaned = scrub.describe(echo);
    assert!(
        !cleaned.contains("sk-known-key"),
        "확인 가리기 후 키: {cleaned}"
    );
    assert!(
        !cleaned.contains("127.0.0.1"),
        "확인 가리기 후 호스트: {cleaned}"
    );
    // 가림 도구 자신의 Display·Debug에도 원문이 없습니다.
    assert!(!format!("{scrub:?}").contains("sk-known-key"));
    assert!(!format!("{scrub}").contains("sk-known-key"));
}
