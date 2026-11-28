//! T8 시각 시험: `StreamTimings` 세 필드의 순서와 토큰 없는 경우만 봄.
use crate::server::MockServer;
use crate::stream_open::CHUNK_GAP;
use crate::stream_open::NO_GAP;
use crate::stream_open::outcome_from;
use crate::usage_stream::chunks_with_usage;
use crate::usage_stream::usage_only_chunks;

#[tokio::test]
async fn timings_keep_request_start_before_first_and_last_token() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let outcome = outcome_from(&server, chunks_with_usage(), CHUNK_GAP).await;

    let first = outcome
        .timings
        .first_token
        .expect("first token time is recorded");
    let last = outcome
        .timings
        .last_token
        .expect("last token time is recorded");
    assert!(
        outcome.timings.request_start < first,
        "request start comes before first token: start {:?}, first {first:?}",
        outcome.timings.request_start
    );
    assert!(
        first < last,
        "first token comes before last token: first {first:?}, last {last:?}"
    );

    server.stop();
}

#[tokio::test]
async fn stream_without_tokens_leaves_first_and_last_empty() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let outcome = outcome_from(&server, usage_only_chunks(), NO_GAP).await;

    assert!(
        outcome.timings.first_token.is_none(),
        "no token means no first token time"
    );
    assert!(
        outcome.timings.last_token.is_none(),
        "no token means no last token time"
    );

    server.stop();
}
