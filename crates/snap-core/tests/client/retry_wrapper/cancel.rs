//! 02-4 재시도 래퍼 취소 시험. 백오프 대기 중 취소가 취소 오류로 끝나고 다시 열지 않는지 봄.

use std::time::Duration;
use std::time::Instant;

use snap_core::llm::LlmClient;
use snap_core::llm::TransportError;
use tokio_util::sync::CancellationToken;

use crate::cancel::materials::cancel_after;
use crate::request::request_with_user_message;
use crate::retry_wrapper::materials::ONE_REQUEST;
use crate::retry_wrapper::materials::SETTINGS;
use crate::retry_wrapper::materials::wrapper;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;
use crate::stream_open::RETRY_AFTER;

/// 백오프 취소 시험이 권유하는 대기 초. 시험이 취소로 끝나게 넉넉하게 둔다.
const LONG_RETRY_AFTER_SECONDS: u64 = 30;

/// 백오프 대기 중 취소 시각.
const BACKOFF_CANCEL_AT: Duration = Duration::from_millis(60);

/// 백오프 취소를 확인하는 시간 상한. 권유 대기는 훨씬 길게 두므로 상한은 취소가 먼저 난다.
const BACKOFF_CANCEL_BOUND: Duration = Duration::from_millis(2_000);

#[tokio::test]
async fn cancel_during_backoff_ends_as_cancel_error_without_reopening() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(
        MockScript::with_status(429)
            .with_header(RETRY_AFTER, &LONG_RETRY_AFTER_SECONDS.to_string())
            .with_body(vec![StreamStep::chunk("rate limit".to_owned())]),
    );

    let cancel = CancellationToken::new();
    cancel_after(&cancel, BACKOFF_CANCEL_AT);

    let started = Instant::now();
    #[allow(clippy::expect_used)]
    let error = wrapper(&server, SETTINGS)
        .open(&request_with_user_message(), cancel)
        .await
        .expect_err("cancel during backoff ends the wrapper");
    let elapsed = started.elapsed();
    let requests = server.requests().snapshot().len();

    assert_eq!(
        TransportError::cancelled(),
        error,
        "backoff cancel reports the cancel error: {error}"
    );
    assert!(
        elapsed < BACKOFF_CANCEL_BOUND,
        "backoff cancel ends long before the header hint: {elapsed:?}"
    );
    assert_eq!(
        requests, ONE_REQUEST,
        "backoff cancel opens no extra request"
    );

    server.stop();
}
