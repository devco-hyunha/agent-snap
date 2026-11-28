//! T7 보정 시험: HTTP 200 안 최상위 `error` 청크로 서버가 키를 되돌려 주는 경로의 2차 가리기만 봄.
//!
//! 관찰 지점은 스트림 오류 요약(`RedactedSummary`)다. 전송 오류 Display·Debug는 요약 값을 구조상 담지 않아 2차 가리기를 볼 수 없다.

use snap_core::config::SecretString;
use snap_core::llm::LlmClient;
use snap_core::llm::RedactedSummary;
use snap_core::llm::SseError;
use snap_core::llm::TransportError;
use snap_core::locale::config::SECRET_REDACTED;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::server::MockServer;
use crate::stream_echo::ECHO_KEY;
use crate::stream_echo::echo_stream_script_for_host;
use crate::stream_echo::echo_transport;

/// 스트림 오류를 받아냄.
#[allow(clippy::expect_used)]
async fn stream_error(server: &MockServer, key: Option<&SecretString>) -> TransportError {
    echo_transport(server, key)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("server error chunk ends the stream")
}

/// 전송 오류에서 서버 요약만 꺼냄. 2차 가리기 관찰 지점.
fn stream_summary(error: &TransportError) -> &RedactedSummary {
    let TransportError::Stream(SseError::Stream { summary }) = &error else {
        panic!("test expects stream error: {error}");
    };
    summary
}

/// 모의 서버 주소의 호스트. 2차 가리기 바늘과 같은 값.
#[allow(clippy::unwrap_used)]
fn mock_host(server: &MockServer) -> String {
    server.address().ip().to_string()
}

#[tokio::test]
async fn stream_error_hides_key_echoed_by_server() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let host = mock_host(&server);
    server.enqueue(echo_stream_script_for_host(&host));

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = stream_error(&server, Some(&key)).await;

    let summary = stream_summary(&error).to_string();
    assert!(
        !summary.contains(ECHO_KEY),
        "stream summary hides echoed key: {summary}"
    );
    assert!(
        !summary.contains(&host),
        "stream summary hides echoed host: {summary}"
    );
    assert!(
        summary.contains(SECRET_REDACTED),
        "stream summary carries the redacted marker: {summary}"
    );

    server.stop();
}

#[tokio::test]
async fn stream_error_hides_base_url_host_echoed_by_server() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    // 모의 서버 주소의 호스트가 서버 오류 문구에 그대로 실리도록 재료를 맞춤.
    let host = mock_host(&server);
    server.enqueue(echo_stream_script_for_host(&host));

    let key = SecretString::new(format!("{ECHO_KEY}@{host}"));
    let error = stream_error(&server, Some(&key)).await;

    let summary = stream_summary(&error).to_string();
    assert!(
        !summary.contains(&host),
        "stream summary hides base_url host: {summary}"
    );
    assert!(
        summary.contains(SECRET_REDACTED),
        "base_url host is replaced by the redacted marker: {summary}"
    );

    server.stop();
}

#[tokio::test]
async fn stream_error_keeps_stream_kind_and_redacted_marker() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let host = mock_host(&server);
    server.enqueue(echo_stream_script_for_host(&host));

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = stream_error(&server, Some(&key)).await;

    let summary = stream_summary(&error).to_string();
    assert!(
        summary.contains(SECRET_REDACTED),
        "echoed stream summary is redacted: {summary}"
    );

    server.stop();
}

#[tokio::test]
async fn keyless_stream_error_still_hides_echoed_host() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    // 키가 없으면 2차 가리기 바늘은 base_url 호스트뿐이라, 그 호스트를 오류 문구에 실음.
    let host = mock_host(&server);
    server.enqueue(echo_stream_script_for_host(&host));

    let error = stream_error(&server, None).await;

    let summary = stream_summary(&error).to_string();
    assert!(
        !summary.contains(&host),
        "keyless transport still hides echoed host: {summary}"
    );
    assert!(
        summary.contains(SECRET_REDACTED),
        "keyless summary carries the redacted marker: {summary}"
    );

    server.stop();
}
