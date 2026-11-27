//! T7 전송 오류 가리기 시험: 서버가 되돌린 키·호스트를 2차로 가린 결과만 봄.
//!
//! 시험용 키·호스트 값은 어림 가리기를 그대로 지나, 2차 가리기로만 지워지는 형태다.

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

/// 어림 가리기를 그대로 지나가는 시험용 호스트 문자열.
const ECHO_HOST: &str = "api.echo-host.invalid";

/// 2차 가리기로만 지워지는 값 두 종을 되돌려 주는 본문.
fn echoed_body() -> String {
    format!("upstream {ECHO_HOST} failed for {ECHO_KEY} with quota exceeded")
}

/// 키가 있는 전송으로 요청을 보내고 상태를 받아냄.
#[allow(clippy::expect_used)]
async fn open_error(server: &MockServer, key: Option<&SecretString>) -> TransportError {
    echo_transport(server, key)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("server error status ends the stream")
}

/// 지정한 상태 코드로 오류 본문을 되돌리는 응답 스크립트.
fn echo_script(status: u16) -> MockScript {
    MockScript::with_status(status).with_body(vec![StreamStep::chunk(echoed_body())])
}

#[tokio::test]
async fn status_error_hides_key_echoed_by_server() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(echo_script(500));

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = open_error(&server, Some(&key)).await;

    let display = error.to_string();
    let debug = format!("{error:?}");
    for secret in [ECHO_KEY, ECHO_HOST] {
        assert!(
            !display.contains(secret) && !debug.contains(secret),
            "status error hides echoed value: {display}"
        );
    }

    server.stop();
}

#[tokio::test]
async fn status_error_keeps_code_and_redacted_marker() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(echo_script(500));

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = open_error(&server, Some(&key)).await;

    let TransportError::Status { status, summary } = &error else {
        panic!("test expects status error: {error}");
    };
    assert_eq!(*status, 500, "status carries the code");
    assert!(
        summary.to_string().contains("가려짐"),
        "echoed body is redacted: {summary}"
    );

    server.stop();
}

#[tokio::test]
async fn status_error_summary_stays_inside_length_cap() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let long = format!("{} {ECHO_KEY}", "상세 ".repeat(400));
    server.enqueue(MockScript::with_status(503).with_body(vec![StreamStep::chunk(long)]));

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = open_error(&server, Some(&key)).await;

    let TransportError::Status { summary, .. } = &error else {
        panic!("test expects status error: {error}");
    };
    assert!(
        summary.to_string().chars().count() <= snap_core::llm::SUMMARY_MAX_CHARS,
        "summary stays inside the cap"
    );
    assert!(
        !summary.to_string().contains(ECHO_KEY),
        "long body hides key"
    );

    server.stop();
}

#[tokio::test]
async fn keyless_transport_hides_echoed_host() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(echo_script(500));

    let error = open_error(&server, None).await;

    let display = error.to_string();
    assert!(
        !display.contains(ECHO_HOST),
        "keyless transport still hides echoed host: {display}"
    );

    server.stop();
}
