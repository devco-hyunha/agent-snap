//! 서버 오류 본문·연결 오류 표시에서 알고 있는 키·호스트 가리기 통합 테스트(AC 2·6).

use axum::http::StatusCode;

use super::super::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use super::super::support::sample_request;
use super::client_at_with;
use super::test_http;
use crate::llm::error::redact::summarize_stream_error;
use crate::llm::{CancellationToken, LlmClient, TransportError};

// AC 2(D28)·AC 6: 서버 오류 본문과 연결 실패에서 알고 있는 키·호스트를 한 번 더 가린다.
#[tokio::test]
async fn server_status_body_hides_known_key_and_host() {
    let key = "sk-leak-key-value-1234567890abcd";
    // 어림 규칙을 통과하는 자리(구분 문자가 섞여 키 덩어리를 찾지 못함)에 키를 심습니다.
    let body = format!("upstream echo: key {key} at 127.0.0.1:6009 /v1 chat completions failed");
    let server = MockServer::spawn(MockResponseScript {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        body_parts: vec![MockBodyPart::immediate(body.as_bytes().to_vec())],
        ..MockResponseScript::empty_ok()
    })
    .await;
    let host = server
        .base_url()
        .strip_prefix("http://")
        .expect("로컬 주소")
        .to_owned();

    let error = client_at_with(server.base_url(), Some(key), test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect_err("서버 오류");
    assert!(matches!(
        error,
        TransportError::ServerStatus { status: 500, .. }
    ));
    let display = format!("{error}");
    let debug = format!("{error:?}");
    assert!(!display.contains(key), "Display에 키가 있습니다: {display}");
    assert!(!debug.contains(key), "Debug에 키가 있습니다: {debug}");
    assert!(!display.contains(&host));
    assert!(!debug.contains(&host));
    assert!(display.contains("500"));
}

// AC 2: 오류의 Display·Debug에 키와 base_url 호스트가 없다(취소·본문 읽기 실패 포함).
#[test]
fn transport_errors_hide_secrets_in_display_and_debug() {
    let key = "sk-hidden-key-value-1234567890abcdE";
    let errors = [
        TransportError::Cancelled,
        TransportError::BodyRead,
        TransportError::ConnectFailed {
            kind: crate::llm::PreHeaderConnectKind::Tls,
            connect_seconds: 10,
        },
        TransportError::Timeout {
            segment: crate::llm::TimeoutSegment::StreamIdle,
            seconds: 120,
        },
        TransportError::ServerStatus {
            status: 429,
            body_summary: summarize_stream_error(&format!(
                "rate limited by https://127.0.0.1:6050/v1 for {key} after 429"
            )),
        },
    ];
    for error in errors {
        let display = format!("{error}");
        let debug = format!("{error:?}");
        assert!(!display.contains(key));
        assert!(!debug.contains(key));
        assert!(!debug.contains("127.0.0.1"));
    }
}
