//! T6 시간 초과 시험: 헤더 대기·스트림 유휴를 따로 재는 동작만 봄.
//!
//! 시험용 시간 값은 짧게 주고, 경과 시간은 넉넉한 하한·상한으로만 확인.

use std::time::Duration;
use std::time::Instant;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::LlmClient;
use snap_core::llm::StreamReceiver;
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

/// 시험용 연결 시간.
const CONNECT: Duration = Duration::from_millis(500);

/// 시험용 스트림 유휴 시간.
const IDLE: Duration = Duration::from_millis(120);

/// 시험용 헤더 대기 시간.
const HEADER_WAIT: Duration = Duration::from_millis(600);

/// 헤더 지연이 유휴 시간보다 늦게 오는 값.
const LATE_HEADER: Duration = Duration::from_millis(300);

/// 시험용 시간 값 묶음.
fn timings(header_wait: Duration) -> TransportTimings {
    transport_timings(CONNECT, header_wait, IDLE)
}

/// 모의 서버로 향하는 전송 구현.
fn transport(base_url: &str, header_wait: Duration) -> HttpTransport {
    HttpTransport::with_timings(base_url, EndpointKind::Openai, None, timings(header_wait))
}

#[tokio::test]
async fn header_later_than_idle_setting_succeeds_within_header_wait() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::ok()
            .with_header_delay(LATE_HEADER)
            .with_body(vec![StreamStep::chunk(FIRST_CHUNK)]),
    );

    let started = Instant::now();
    let result = transport(&base_url(&server), HEADER_WAIT)
        .open(&request_with_user_message(), CancellationToken::new())
        .await;
    let elapsed = started.elapsed();

    let mut receiver = result.expect("late header inside header wait succeeds");
    let outcome = receiver
        .finish()
        .expect("stream finishes after late header");
    assert_eq!(
        outcome.delta.content, "가",
        "stream restores after late header"
    );
    assert!(
        elapsed >= LATE_HEADER,
        "header arrives no earlier than the script delay: {elapsed:?}"
    );
    assert!(
        elapsed < HEADER_WAIT,
        "success stays inside the header wait bound: {elapsed:?}"
    );

    server.stop();
}

#[tokio::test]
async fn header_never_arrives_ends_as_header_wait_timeout() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_header_delay(STOPPED_BODY));

    let started = Instant::now();
    let error = transport(&base_url(&server), HEADER_WAIT)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("header that never arrives times out");
    let elapsed = started.elapsed();

    assert!(
        matches!(error, TransportError::HeaderWait { .. }),
        "header that never arrives reports header wait: {error}"
    );
    assert!(
        elapsed >= HEADER_WAIT,
        "header wait fires no earlier than the setting: {elapsed:?}"
    );
    assert!(
        elapsed < STOPPED_BODY,
        "header wait fires before the header delay ends: {elapsed:?}"
    );

    server.stop();
}

#[tokio::test]
async fn stopped_body_ends_as_stream_idle_timeout() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk(FIRST_CHUNK).chunk_after(STOPPED_BODY),
        StreamStep::chunk(SECOND_CHUNK),
    ]));

    let started = Instant::now();
    let error = transport(&base_url(&server), HEADER_WAIT)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("stopped body times out");
    let elapsed = started.elapsed();

    assert!(
        matches!(error, TransportError::Idle { .. }),
        "stopped body reports stream idle: {error}"
    );
    assert!(
        elapsed >= IDLE,
        "idle fires no earlier than the setting: {elapsed:?}"
    );
    assert!(
        elapsed < STOPPED_BODY,
        "idle fires before the body resumes: {elapsed:?}"
    );

    server.stop();
}

#[tokio::test]
async fn header_wait_timeout_error_carries_setting_seconds() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_header_delay(STOPPED_BODY));

    let header_wait = Duration::from_millis(80);
    let error = transport(&base_url(&server), header_wait)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("header wait times out");

    assert_eq!(
        TransportError::header_wait(header_wait.as_secs_f64()),
        error,
        "header wait error carries the setting seconds"
    );

    server.stop();
}

#[tokio::test]
async fn idle_timeout_error_carries_setting_seconds() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk(FIRST_CHUNK).chunk_after(STOPPED_BODY),
    ]));

    let error = transport(&base_url(&server), HEADER_WAIT)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("stopped body times out");

    assert_eq!(
        TransportError::idle(IDLE.as_secs_f64()),
        error,
        "idle error carries the setting seconds"
    );

    server.stop();
}
