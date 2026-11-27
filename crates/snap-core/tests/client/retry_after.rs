//! 상태 오류의 `Retry-After` 헤더 전달 시험. 헤더 있음·없음과 2차 가리기만 담음.

use snap_core::config::SecretString;
use snap_core::llm::LlmClient;
use snap_core::llm::TransportError;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;
use crate::stream_echo::ECHO_KEY;
use crate::stream_echo::echo_transport;
use crate::stream_open::RETRY_AFTER;

/// 키가 있는 전송으로 요청을 보내고 상태 오류를 받아냄.
#[allow(clippy::expect_used)]
async fn status_error_with_key(server: &MockServer, key: &SecretString) -> TransportError {
    echo_transport(server, Some(key))
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("server error status ends the stream")
}

/// 상태 오류에서 `Retry-After` 값만 꺼냄.
fn retry_after_of(error: &TransportError) -> Option<String> {
    let TransportError::Status { retry_after, .. } = error else {
        panic!("test expects status error: {error}");
    };
    retry_after.as_ref().map(|value| value.expose().to_owned())
}

#[tokio::test]
async fn status_error_carries_retry_after_header_value() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::with_status(429)
            .with_header(RETRY_AFTER, "12")
            .with_body(vec![StreamStep::chunk("rate limit".to_owned())]),
    );

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = status_error_with_key(&server, &key).await;

    assert_eq!(
        retry_after_of(&error).as_deref(),
        Some("12"),
        "header value reaches the status error"
    );

    server.stop();
}

#[tokio::test]
async fn status_error_without_retry_after_header_is_none() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::with_status(429));

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = status_error_with_key(&server, &key).await;

    assert_eq!(retry_after_of(&error), None, "absent header stays none");

    server.stop();
}

#[tokio::test]
async fn status_error_hides_key_echoed_in_retry_after_header() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::with_status(429)
            .with_header(RETRY_AFTER, ECHO_KEY)
            .with_body(vec![StreamStep::chunk("rate limit".to_owned())]),
    );

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = status_error_with_key(&server, &key).await;

    let display = error.to_string();
    let debug = format!("{error:?}");
    let value = retry_after_of(&error).unwrap_or_default();
    assert!(
        !display.contains(ECHO_KEY) && !debug.contains(ECHO_KEY) && !value.contains(ECHO_KEY),
        "header value is redacted: {value}"
    );

    server.stop();
}
