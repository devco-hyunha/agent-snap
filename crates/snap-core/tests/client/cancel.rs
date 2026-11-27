//! T6 취소 시험: 스트림 도중 취소하면 취소 오류와 연결 정리만 봄.
//!
//! 시험용 시간 값은 짧게 주고, 경과 시간은 넉넉한 하한·상한으로만 확인.

use std::time::Duration;
use std::time::Instant;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::LlmClient;
use snap_core::llm::TransportError;
use snap_core::llm::TransportTimings;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;
use crate::stream_open::base_url;
use crate::stream_open::transport_timings;
use crate::stream_payload::FIRST_CHUNK;
use crate::stream_payload::SECOND_CHUNK;
use crate::stream_payload::STOPPED_BODY;

/// 취소 오류가 나야 하는 시간 상한.
const CANCEL_BOUND: Duration = Duration::from_millis(500);

/// 서버 쪽 연결이 닫혀야 하는 시간 상한.
const CLOSE_BOUND: Duration = Duration::from_millis(3_000);

/// 취소가 나가는 시각.
const CANCEL_AT: Duration = Duration::from_millis(60);

/// 취소 시험용 짧은 시간 값 묶음.
fn timings() -> TransportTimings {
    transport_timings(
        Duration::from_millis(500),
        Duration::from_millis(600),
        STOPPED_BODY,
    )
}

/// 조각을 하나 보낸 뒤 본문이 멈춘 스트림 스크립트.
fn stalled_after_one_chunk() -> MockScript {
    MockScript::ok().with_body(vec![
        StreamStep::chunk(FIRST_CHUNK).chunk_after(STOPPED_BODY),
        StreamStep::chunk(SECOND_CHUNK),
    ])
}

/// 취소 시각이 지나면 끊기는 취소 토큰을 만들어 배경으로 띄움.
fn cancel_after(token: &CancellationToken, wait: Duration) {
    let signal = token.clone();
    tokio::spawn(async move {
        tokio::time::sleep(wait).await;
        signal.cancel();
    });
}

#[tokio::test]
async fn cancel_mid_stream_reports_one_cancel_error_quickly() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(stalled_after_one_chunk());

    let cancel = CancellationToken::new();
    cancel_after(&cancel, CANCEL_AT);
    let transport =
        HttpTransport::with_timings(&base_url(&server), EndpointKind::Openai, None, timings());

    let started = Instant::now();
    let error = transport
        .open(&request_with_user_message(), cancel)
        .await
        .expect_err("cancel mid stream ends the stream");
    let elapsed = started.elapsed();

    assert_eq!(
        TransportError::cancelled(),
        error,
        "cancel reports cancel error"
    );
    assert!(
        elapsed < CANCEL_BOUND,
        "cancel error arrives inside the bound: {elapsed:?}"
    );

    server.stop();
}

#[tokio::test]
async fn cancel_closes_server_connection_inside_bound() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(stalled_after_one_chunk());

    let cancel = CancellationToken::new();
    cancel_after(&cancel, CANCEL_AT);
    let transport =
        HttpTransport::with_timings(&base_url(&server), EndpointKind::Openai, None, timings());

    let _ = transport
        .open(&request_with_user_message(), cancel)
        .await
        .expect_err("cancel ends the stream");

    let started = Instant::now();
    while !server.is_connection_closed() {
        assert!(
            started.elapsed() < CLOSE_BOUND,
            "server connection closes inside the bound"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    server.stop();
}

#[tokio::test]
async fn cancel_before_open_ends_as_cancel_error() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(stalled_after_one_chunk());

    let cancel = CancellationToken::new();
    cancel.cancel();
    let transport =
        HttpTransport::with_timings(&base_url(&server), EndpointKind::Openai, None, timings());

    let error = transport
        .open(&request_with_user_message(), cancel)
        .await
        .expect_err("cancel before open ends the stream");

    assert_eq!(
        TransportError::cancelled(),
        error,
        "cancel before open reports cancel error"
    );

    server.stop();
}
