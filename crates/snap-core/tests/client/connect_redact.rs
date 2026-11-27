//! T7 헤더 전 연결 가리기 시험: 연결 실패 오류 문구에 엔드포인트 호스트가 없는지 확인만 함.

use std::time::Duration;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::LlmClient;
use snap_core::llm::TransportTimings;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::server::MockServer;
use crate::stream_open::transport_timings;

/// 시험용 연결 시간. 연결 단계 실패가 시간 초과의 그릇된 표시로 잡히지 않게 넉넉하게 둔다.
const CONNECT: Duration = Duration::from_secs(5);

/// 시험용 시간 값 묶음.
fn timings() -> TransportTimings {
    transport_timings(CONNECT, Duration::from_secs(2), Duration::from_secs(2))
}

#[tokio::test]
async fn connect_failure_hides_endpoint_host_in_display_and_debug() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let address = server.address();
    server.stop();
    tokio::time::sleep(Duration::from_millis(120)).await;

    let transport = HttpTransport::with_timings(
        &format!("http://{address}/v1"),
        EndpointKind::Openai,
        None,
        timings(),
    );
    let error = transport
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("stopped server fails before headers");

    let display = error.to_string();
    let debug = format!("{error:?}");
    let host = address.ip().to_string();
    assert!(
        !display.contains(&host) && !debug.contains(&host),
        "connect error hides the endpoint host: {display}"
    );
}
