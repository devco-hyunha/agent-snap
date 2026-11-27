//! 본문 멈춤·연결 종료 제어 확인. 스트림 규칙이 끊기는 지점을 시험이 확인한다.
use std::time::Duration;

use crate::fixtures::ChunkRead;
use crate::fixtures::RawClient;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;

/// 첫 조각을 읽을 때 쓰는 넉넉한 대기 하한.
const FIRST_WAIT: Duration = Duration::from_millis(600);

/// 본문이 멈췄다고 판단하는 짧은 대기.
const IDLE_WAIT: Duration = Duration::from_millis(80);

/// 조각을 다 보낸 뒤 끊으면, 시험은 조각을 받고 연결이 닫힘을 확인한다.
#[tokio::test]
async fn cut_after_chunks_closes_connection() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::ok()
            .with_body(vec![StreamStep::chunk("data: first\n\n")])
            .with_stop(StreamStop::Cut),
    );

    check_connection_closes_after_chunk(&server, b"data: first\n\n").await;
    server.stop();
}

/// 본문 없이 연결만 끊으면 시험은 조각 없이 닫힘을 확인한다.
#[tokio::test]
async fn drop_without_chunks_closes_connection() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_stop(StreamStop::Drop));

    check_connection_closes_without_chunk(&server).await;
    server.stop();
}

/// 조각 뒤에 긴 대기를 주면 본문이 멈춘 상태로 남고, 시험은 짧은 대기로 멈춤을 확인한다.
#[tokio::test]
async fn long_gap_after_chunk_stops_body() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk("data: first\n\n").chunk_after(Duration::from_millis(600)),
        StreamStep::chunk("data: never\n\n"),
    ]));

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let head = client
        .send("POST", "/v1/chat/completions", &[], "")
        .await
        .unwrap();
    assert_eq!(head.status, 200);

    #[allow(clippy::unwrap_used)]
    let first = client.read_chunk(FIRST_WAIT).await.unwrap();
    assert_eq!(first, ChunkRead::Chunk(b"data: first\n\n".to_vec()));

    #[allow(clippy::unwrap_used)]
    let stopped = client.read_chunk(IDLE_WAIT).await.unwrap();
    assert_eq!(stopped, ChunkRead::Timeout);

    server.stop();
}

/// 본문 규칙 중간에 멈춤 규칙을 두면 그 뒤 조각은 나가지 않고 연결이 닫힌다.
#[tokio::test]
async fn stop_rule_in_body_closes_connection() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk("data: first\n\n"),
        StreamStep::Stop(StreamStop::Cut),
        StreamStep::chunk("data: never\n\n"),
    ]));

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let head = client
        .send("POST", "/v1/chat/completions", &[], "")
        .await
        .unwrap();
    assert_eq!(head.status, 200);

    #[allow(clippy::unwrap_used)]
    let first = client.read_chunk(FIRST_WAIT).await.unwrap();
    assert_eq!(first, ChunkRead::Chunk(b"data: first\n\n".to_vec()));

    #[allow(clippy::unwrap_used)]
    let closed = client.read_chunk(FIRST_WAIT).await.unwrap();
    assert_eq!(closed, ChunkRead::End);

    server.stop();
}

/// 정상 종료 규칙은 끝 표시로 끝나고, 이후 연결은 닫힌다.
#[tokio::test]
async fn end_after_chunks_closes_connection() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::ok()
            .with_body(vec![StreamStep::chunk("data: last\n\n")])
            .with_stop(StreamStop::End),
    );

    check_connection_closes_after_chunk(&server, b"data: last\n\n").await;
    server.stop();
}

/// 조각을 받고 나서 연결이 닫히는지 확인한다.
async fn check_connection_closes_after_chunk(server: &MockServer, expected: &[u8]) {
    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    #[allow(clippy::unwrap_used)]
    let head = client
        .send("POST", "/v1/chat/completions", &[], "")
        .await
        .unwrap();
    assert_eq!(head.status, 200);

    #[allow(clippy::unwrap_used)]
    let chunk = client.read_chunk(FIRST_WAIT).await.unwrap();
    assert_eq!(chunk, ChunkRead::Chunk(expected.to_vec()));

    #[allow(clippy::unwrap_used)]
    let closed = client.read_chunk(FIRST_WAIT).await.unwrap();
    assert_eq!(closed, ChunkRead::End);
}

/// 조각 없이 연결이 닫히는지 확인한다.
async fn check_connection_closes_without_chunk(server: &MockServer) {
    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    #[allow(clippy::unwrap_used)]
    let head = client
        .send("POST", "/v1/chat/completions", &[], "")
        .await
        .unwrap();
    assert_eq!(head.status, 200);

    #[allow(clippy::unwrap_used)]
    let closed = client.read_chunk(FIRST_WAIT).await.unwrap();
    assert_eq!(closed, ChunkRead::End);
}
