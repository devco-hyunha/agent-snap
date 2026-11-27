//! T7 헤더 전 연결 세부 구분 시험: reqwest 연결 오류가 종류와 초 값으로 담기는지만 봄.
//!
//! 세부 여섯 종류 전체의 Display·Debug 가리기는 `error.rs` 시험이 맡고,
//! 이 파일은 실제 연결 실패로 만들 수 있는 DNS·재설정 경로와 초 값 확인만 한다.

use std::time::Duration;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::LlmClient;
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::TransportError;
use snap_core::llm::TransportTimings;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::stream_open::transport_timings;

/// 시험용 연결 시간. 연결 단계 실패가 시간 초과의 그릇된 표시로 잡히지 않게 넉넉하게 둔다.
const CONNECT: Duration = Duration::from_secs(5);

/// 시험용 시간 값 묶음.
fn timings() -> TransportTimings {
    transport_timings(CONNECT, Duration::from_secs(2), Duration::from_secs(2))
}

#[tokio::test]
async fn unresolvable_host_maps_to_dns_kind() {
    let transport = HttpTransport::with_timings(
        "http://snap-name-that-does-not-resolve.invalid/v1",
        EndpointKind::Openai,
        None,
        timings(),
    );

    let error = transport
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("unresolvable host fails before headers");

    assert!(
        matches!(
            error,
            TransportError::Connect {
                kind: PreHeaderConnectKind::Dns,
                connect_seconds
            } if connect_seconds == CONNECT.as_secs_f64()
        ),
        "unresolvable host reports dns kind with setting seconds: {error}"
    );
}

#[tokio::test]
async fn connection_dropped_by_server_maps_to_reset_kind() {
    // 연결을 받아 즉시 끊는 서버는 헤더 전에 연결이 끊긴 실패로 잡힌다.
    let any_port: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    #[allow(clippy::unwrap_used)]
    let listener = TcpListener::bind(any_port).await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                return;
            };
            drop(stream);
        }
    });

    let transport = HttpTransport::with_timings(
        &format!("http://{address}/v1"),
        EndpointKind::Openai,
        None,
        timings(),
    );

    let error = transport
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("server that drops the connection fails before headers");

    assert!(
        matches!(
            error,
            TransportError::Connect {
                kind: PreHeaderConnectKind::Reset,
                connect_seconds
            } if connect_seconds == CONNECT.as_secs_f64()
        ),
        "dropped connection reports reset kind: {error:?}"
    );
}
