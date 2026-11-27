//! 02-7 T2 재시도 래퍼 확인 시험. 조립 결과가 래퍼를 포함하는지 한 책임만 봄.

use snap_core::llm::LlmClient;
use snap_core::llm::StreamReceiver;
use tokio_util::sync::CancellationToken;

use crate::build::materials::bundle_for;
use crate::request::request_with_user_message;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;
use crate::stream_payload::FIRST_CHUNK;

/// 조립된 클라이언트가 429를 만나면 시도를 하나 더 연다. 래퍼가 감겨 있어야 가능한 동작.
#[tokio::test]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn assembled_client_reopens_after_rate_limit() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::with_status(429));
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk(FIRST_CHUNK),
        StreamStep::Stop(StreamStop::End),
    ]));

    let bundle = bundle_for(&server, "sk-buildvalue123");
    let mut receiver = bundle
        .client
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect("wrapped client reopens after rate limit");
    let outcome = receiver.finish().expect("reopened stream finishes");

    assert_eq!(outcome.delta.content, "가", "reopened stream restores body");
    assert_eq!(
        server.requests().snapshot().len(),
        2,
        "retry wrapper opens the request twice"
    );

    server.stop();
}
