//! 헤더 지연 제어 확인. 헤더가 늦게 오는 시점과 즉시 오는 시점을 나눈다.
use std::time::Duration;

use tokio::time::Instant;

use crate::fixtures::ChunkRead;
use crate::fixtures::RawClient;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;

/// 시험용 헤더 지연.
const HEADER_DELAY: Duration = Duration::from_millis(120);

/// 본문 조각을 읽을 때 쓰는 넉넉한 대기 하한.
const READ_WAIT: Duration = Duration::from_millis(600);

#[tokio::test]
async fn header_delay_reaches_client_after_delay() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::ok()
            .with_header_delay(HEADER_DELAY)
            .with_body(vec![StreamStep::chunk("data: ok\n\n")]),
    );

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let started = Instant::now();
    #[allow(clippy::unwrap_used)]
    let head = client
        .send("POST", "/v1/chat/completions", &[], "")
        .await
        .unwrap();
    let elapsed = started.elapsed();

    assert_eq!(head.status, 200);
    assert!(
        elapsed >= HEADER_DELAY,
        "header arrives no earlier than the script delay: {elapsed:?}"
    );
    assert!(
        head.headers
            .iter()
            .any(|(name, value)| name == "content-type" && value == "text/event-stream")
    );

    #[allow(clippy::unwrap_used)]
    let chunk = client.read_chunk(READ_WAIT).await.unwrap();
    assert_eq!(chunk, ChunkRead::Chunk(b"data: ok\n\n".to_vec()));

    server.stop();
}

#[tokio::test]
async fn header_without_delay_reaches_client_immediately() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let started = Instant::now();
    #[allow(clippy::unwrap_used)]
    let head = client.send("GET", "/v1/models", &[], "").await.unwrap();
    let elapsed = started.elapsed();

    assert_eq!(head.status, 200);
    assert!(
        elapsed < HEADER_DELAY,
        "immediate header stays below the delay bound: {elapsed:?}"
    );

    server.stop();
}
