//! 본문 멈춤 확인. 스트림 규칙이 대기로 멈추고 규칙 멈춤·급히 끊기로 끝나는 지점만 본다.
use std::time::Duration;

use crate::fixtures::ChunkRead;
use crate::fixtures::RawClient;
use crate::script::ABORT_GAP;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;

/// 본문이 멈췄다고 판단하는 짧은 대기.
const IDLE_WAIT: Duration = Duration::from_millis(80);

/// 첫 조각을 읽을 때 쓰는 넉넉한 대기 하한.
const FIRST_WAIT: Duration = Duration::from_millis(600);

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

/// 급히 끊기 규칙은 헤더를 보낸 뒤 본문 끝 표시 없이 읽기를 실패로 끝낸다.
///
/// 첫 조각 뒤 짧은 대기를 두어 헤더와 조각이 클라이언트로 옮겨간 뒤에 끊긴다.
#[tokio::test]
async fn abort_after_header_fails_body_read() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::ok()
            .with_body(vec![
                StreamStep::chunk("data: first\n\n").chunk_after(ABORT_GAP),
            ])
            .with_stop(StreamStop::Abort),
    );

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    #[allow(clippy::unwrap_used)]
    let head = client
        .send("POST", "/v1/chat/completions", &[], "")
        .await
        .unwrap();
    assert_eq!(head.status, 200, "header arrives before the abort");

    #[allow(clippy::unwrap_used)]
    let first = client.read_chunk(FIRST_WAIT).await.unwrap();
    assert_eq!(
        first,
        ChunkRead::Chunk(b"data: first\n\n".to_vec()),
        "chunk before the abort arrives"
    );

    let aborted = client.read_chunk(FIRST_WAIT).await;
    assert!(
        !matches!(aborted, Ok(ChunkRead::Chunk(_))),
        "no more body after the abort: {aborted:?}",
    );

    server.stop();
}
