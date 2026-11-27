//! T6 시간 초과 시험의 연결 실패 확인: 서버가 닫힌 뒤 연결이 실패로 잡히는지만 봄.
//!
//! 시간 초과 확인(`timeout.rs`)과 다른 변경 이유인 연결 실패 확인만 담는다.

use std::time::Duration;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::LlmClient;
use snap_core::llm::TransportError;
use snap_core::llm::TransportTimings;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::server::MockServer;
use crate::stream_open::transport_timings;

/// 시험용 시간 값 묶음.
fn timings() -> TransportTimings {
    transport_timings(
        Duration::from_millis(500),
        Duration::from_secs(2),
        Duration::from_secs(2),
    )
}

#[tokio::test]
async fn closed_server_ends_as_connect_error() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let address = server.address();
    server.stop();
    tokio::time::sleep(Duration::from_millis(80)).await;

    let transport = HttpTransport::with_timings(
        &format!("http://{address}/v1"),
        EndpointKind::Openai,
        None,
        timings(),
    );
    let result = transport
        .open(&request_with_user_message(), CancellationToken::new())
        .await;

    assert!(
        matches!(result, Err(TransportError::Connect { .. })),
        "closed server reports connect failure: {result:?}",
    );
}
