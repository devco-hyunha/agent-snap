//! T8 사용량 시험: 스트림 종료 결과의 사용량이 서버 값인지 확인만 함.
use snap_core::llm::Usage;

use crate::server::MockServer;
use crate::stream_open::NO_GAP;
use crate::stream_open::outcome_from;
use crate::usage_stream::chunks_with_usage;
use crate::usage_stream::chunks_without_usage;

#[tokio::test]
async fn usage_chunk_reaches_outcome_as_server_value() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let outcome = outcome_from(&server, chunks_with_usage(), NO_GAP).await;

    let usage = outcome.delta.usage.expect("server usage reaches outcome");
    assert_eq!(
        usage,
        Usage {
            prompt_tokens: Some(3),
            completion_tokens: Some(2),
            total_tokens: Some(5),
        },
        "usage is the server value"
    );

    server.stop();
}

#[tokio::test]
async fn stream_without_usage_chunk_reaches_outcome_as_none() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let outcome = outcome_from(&server, chunks_without_usage(), NO_GAP).await;

    assert!(
        outcome.delta.usage.is_none(),
        "no usage chunk leaves outcome usage empty"
    );

    server.stop();
}
