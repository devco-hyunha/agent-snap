//! 02-4 재시도 래퍼 포기 시험. 재시도 대상 아닌 상태와 시도 횟수 초과 포기만 봄.

use snap_core::llm::LlmClient;
use snap_core::llm::TransportError;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::retry_wrapper::materials::LIMITED_ATTEMPTS;
use crate::retry_wrapper::materials::ONE_REQUEST;
use crate::retry_wrapper::materials::SETTINGS;
use crate::retry_wrapper::materials::wrapper;
use crate::script::MockScript;
use crate::server::MockServer;

#[tokio::test]
async fn non_retry_status_gives_up_without_reopening() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::with_status(404));

    #[allow(clippy::expect_used)]
    let error = wrapper(&server, SETTINGS)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("404 ends as status failure");
    let requests = server.requests().snapshot().len();

    assert!(
        matches!(error, TransportError::Status { status: 404, .. }),
        "404 is the inner status error: {error}"
    );
    assert_eq!(
        requests, ONE_REQUEST,
        "non-candidate status opens no extra request"
    );

    server.stop();
}

#[tokio::test]
async fn attempt_limit_gives_up_with_inner_error() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    for _ in 0..LIMITED_ATTEMPTS {
        server.enqueue(MockScript::with_status(429));
    }

    #[allow(clippy::expect_used)]
    let error = wrapper(&server, SETTINGS)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("rate limit over the limit ends as status failure");
    let requests = server.requests().snapshot().len();

    assert!(
        matches!(error, TransportError::Status { status: 429, .. }),
        "give up returns the inner status error: {error}"
    );
    assert_eq!(
        requests, LIMITED_ATTEMPTS,
        "wrapper stops after the attempt limit"
    );

    server.stop();
}
