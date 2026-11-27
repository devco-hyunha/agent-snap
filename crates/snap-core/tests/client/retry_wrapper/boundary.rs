//! 02-4 재시도 래퍼 경계 시험. 헤더 전 재시도 성공과 헤더 후 비재시도 경계만 봄.

use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::LlmClient;
use snap_core::llm::RetryClient;
use snap_core::llm::StreamReceiver;
use snap_core::llm::TransportError;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::retry_wrapper::materials::CONNECT;
use crate::retry_wrapper::materials::HEADER_WAIT;
use crate::retry_wrapper::materials::IDLE;
use crate::retry_wrapper::materials::LIMITED_ATTEMPTS;
use crate::retry_wrapper::materials::ONE_REQUEST;
use crate::retry_wrapper::materials::SETTINGS;
use crate::retry_wrapper::materials::TWO_REQUESTS;
use crate::retry_wrapper::materials::ZeroSource;
use crate::retry_wrapper::materials::abort_after_first_chunk;
use crate::retry_wrapper::materials::dropped_connection_server;
use crate::retry_wrapper::materials::wrapper;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;
use crate::stream_open::RETRY_AFTER;
use crate::stream_open::transport_timings;
use crate::stream_payload::FIRST_CHUNK;
use crate::stream_payload::STOPPED_BODY;
use crate::stream_payload::data_line;
use crate::stream_payload::error_payload;

/// 429가 권유하는 대기 초.
const RETRY_AFTER_SECONDS: u64 = 1;

/// 헤더 값이 지시하는 대기 시간.
const HEADER_HINT: Duration = Duration::from_secs(RETRY_AFTER_SECONDS);

#[tokio::test]
async fn rate_limited_open_reopens_after_header_hint() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::with_status(429)
            .with_header(RETRY_AFTER, &RETRY_AFTER_SECONDS.to_string())
            .with_body(vec![StreamStep::chunk("rate limit".to_owned())]),
    );
    server.enqueue(MockScript::ok().with_body(vec![StreamStep::chunk(FIRST_CHUNK)]));

    let started = Instant::now();
    let result = wrapper(&server, SETTINGS)
        .open(&request_with_user_message(), CancellationToken::new())
        .await;
    let elapsed = started.elapsed();

    #[allow(clippy::expect_used)]
    let mut receiver = result.expect("rate limited open reopens after the header hint");
    #[allow(clippy::expect_used)]
    let outcome = receiver.finish().expect("reopened stream finishes");
    assert_eq!(
        outcome.delta.content, "가",
        "reopened stream restores the body"
    );
    assert_eq!(
        server.requests().snapshot().len(),
        TWO_REQUESTS,
        "wrapper opens the request twice"
    );
    assert!(
        elapsed >= HEADER_HINT,
        "reopen waits no earlier than the header hint: {elapsed:?}"
    );

    server.stop();
}

#[tokio::test]
async fn pre_header_connect_failure_reopens_inside_settings() {
    let (address, accepted) = dropped_connection_server().await;
    let transport = HttpTransport::with_timings(
        &format!("http://{address}/v1"),
        EndpointKind::Openai,
        None,
        transport_timings(CONNECT, HEADER_WAIT, IDLE),
    );
    let wrapper = RetryClient::new(transport, SETTINGS, ZeroSource);

    #[allow(clippy::expect_used)]
    let error = wrapper
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("dropped connection ends as connect failure");
    let attempts = accepted.load(Ordering::Relaxed);

    assert!(
        matches!(error, TransportError::Connect { .. }),
        "connect failure carries the inner error: {error:?}"
    );
    assert_eq!(
        attempts, LIMITED_ATTEMPTS,
        "connect failure reopens inside the settings limit"
    );
}

#[tokio::test]
async fn stream_idle_failure_is_not_replayed() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk(FIRST_CHUNK).chunk_after(STOPPED_BODY),
    ]));

    #[allow(clippy::expect_used)]
    let error = wrapper(&server, SETTINGS)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("stopped body ends as idle failure");
    let requests = server.requests().snapshot().len();

    assert!(
        matches!(error, TransportError::Idle { .. }),
        "idle failure is the inner error: {error}"
    );
    assert_eq!(requests, ONE_REQUEST, "idle failure opens no extra request");

    server.stop();
}

#[tokio::test]
async fn body_read_failure_after_header_is_not_replayed() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(abort_after_first_chunk());

    #[allow(clippy::expect_used)]
    let error = wrapper(&server, SETTINGS)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("aborted body ends as read failure");
    let requests = server.requests().snapshot().len();

    assert!(
        matches!(error, TransportError::StreamRead { .. }),
        "post-header read failure is its own error: {error}"
    );
    assert_eq!(requests, ONE_REQUEST, "read failure opens no extra request");

    server.stop();
}

#[tokio::test]
async fn server_error_chunk_after_output_is_not_replayed() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::ok()
            .with_body(vec![
                StreamStep::chunk(FIRST_CHUNK),
                StreamStep::chunk(data_line(&error_payload("서버 오류"))),
            ])
            .with_stop(StreamStop::Cut),
    );

    #[allow(clippy::expect_used)]
    let error = wrapper(&server, SETTINGS)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("server error chunk ends the stream");
    let requests = server.requests().snapshot().len();

    assert!(
        matches!(error, TransportError::Stream(_)),
        "stream chunk failure is the inner error: {error}"
    );
    assert_eq!(
        requests, ONE_REQUEST,
        "stream failure opens no extra request"
    );

    server.stop();
}
