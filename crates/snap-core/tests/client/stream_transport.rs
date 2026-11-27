//! T5 전송 통합 시험: 모의 SSE 서버 조각을 수신자에 넣어 복원 결과 확인.

use std::time::Duration;
use std::time::Instant;

use snap_core::llm::RestoreReceiver;
use snap_core::llm::StreamReceiver;
use snap_core::llm::TransportError;

use crate::fixtures::ChunkRead;
use crate::fixtures::RawClient;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;
use crate::stream_payload::data_line;
use crate::stream_payload::delta_payload;
use crate::stream_payload::error_payload;
use crate::stream_payload::mixed_stream;
use serde_json::json;

/// 조각을 읽을 때 쓰는 넉넉한 대기 하한.
const READ_WAIT: Duration = Duration::from_millis(800);

/// 모의 서버 주소를 `/v1`까지 포함한 형태로 만듦.
fn completions_path(server: &MockServer) -> String {
    format!("{}/v1/chat/completions", server.base_url())
        .split_once("://")
        .map(|(_, rest)| {
            let slash = rest.find('/').unwrap_or(0);
            rest[slash..].to_owned()
        })
        .unwrap_or_default()
}

#[tokio::test]
async fn mock_sse_stream_restores_like_02_3() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(sse_steps(&mixed_stream())));

    let mut receiver = RestoreReceiver::new(Instant::now());
    read_into_receiver(&server, &mut receiver)
        .await
        .expect("test reads stream pieces");
    let outcome = receiver.finish().expect("test finishes stream");

    assert_eq!(outcome.delta.content, "가나다", "content restores in order");
    assert_eq!(outcome.delta.reasoning, "생각", "reasoning restores");
    let usage = outcome.delta.usage.expect("usage is read");
    assert_eq!(usage.total_tokens, Some(5), "usage is read at the end");

    server.stop();
}

#[tokio::test]
async fn mock_stream_failure_keeps_failure_after_repush() {
    let content = delta_payload(json!({ "content": "ab" }));
    let stream = format!(
        "{}{}",
        data_line(&content),
        data_line(&error_payload("서버 오류"))
    );

    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::ok()
            .with_body(sse_steps(&stream))
            .with_stop(StreamStop::Cut),
    );

    let mut receiver = RestoreReceiver::new(Instant::now());
    let first = read_into_receiver(&server, &mut receiver).await;

    assert!(first.is_err(), "server error chunk fails the stream");
    let late = receiver.push(b"data: [DONE]\n\n");
    assert!(late.is_err(), "late push is not success");
    assert!(
        receiver.finish().is_err(),
        "finish after failure is not success"
    );

    server.stop();
}

/// SSE 원문을 모의 서버 조각 규칙으로 옮김. 조각 경계는 임의 분할.
fn sse_steps(stream: &str) -> Vec<StreamStep> {
    let bytes = stream.as_bytes();
    let mid = bytes.len() / 2;
    let (first, second) = stream.split_at(mid);
    vec![
        StreamStep::chunk(first.to_owned()),
        StreamStep::chunk(second.to_owned()),
        StreamStep::Stop(StreamStop::End),
    ]
}

/// 모의 서버에서 조각을 읽어 수신자에 넣고, 첫 실패 결과 반환.
#[allow(clippy::expect_used)]
async fn read_into_receiver(
    server: &MockServer,
    receiver: &mut RestoreReceiver,
) -> Result<(), TransportError> {
    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let _head = client
        .send("POST", &completions_path(server), &[], "")
        .await
        .expect("test sends request");

    loop {
        #[allow(clippy::unwrap_used)]
        let read = client.read_chunk(READ_WAIT).await.unwrap();
        match read {
            ChunkRead::Chunk(bytes) => receiver.push(&bytes)?,
            ChunkRead::End | ChunkRead::Timeout => return Ok(()),
        }
    }
}
