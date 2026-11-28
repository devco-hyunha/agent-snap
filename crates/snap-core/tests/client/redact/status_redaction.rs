//! T7 상태 오류 가리기 시험: 서버가 되돌린 키·호스트를 2차로 가린 결과만 봄.

use snap_core::config::SecretString;
use snap_core::llm::TransportError;

use crate::redact::materials::ECHO_HOST;
use crate::redact::materials::echo_script;
use crate::redact::materials::open_error;
use crate::server::MockServer;
use crate::stream_echo::ECHO_KEY;

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

    let TransportError::Status {
        status,
        summary,
        retry_after,
    } = &error
    else {
        panic!("test expects status error: {error}");
    };
    assert_eq!(*status, 500, "status carries the code");
    assert!(
        summary.to_string().contains("가려짐"),
        "echoed body is redacted: {summary}"
    );
    assert!(retry_after.is_none(), "no header stays none");

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
