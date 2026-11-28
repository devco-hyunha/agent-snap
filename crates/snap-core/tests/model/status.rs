//! 02-7 T3 목록 HTTP 오류 시험. 02-6 재시도 규칙(재시도 후보만 재시도)과 오류 가리기를 함께 봄.

use snap_core::llm::ModelsError;
use snap_core::llm::TransportError;

use crate::materials::KEY;
use crate::materials::SETTINGS;
use crate::materials::json_script;
use crate::materials::models_of;
use crate::server::MockServer;

/// 모의 서버에서 조회를 한 번만 해 상태 오류를 받아냄.
#[allow(clippy::expect_used)]
async fn status_of(server: &MockServer) -> ModelsError {
    models_of(server)
        .await
        .expect_err("server status ends the list")
}

/// 429는 재시도 후보라 시도가 늘어난다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_status_429_retries() {
    let server = MockServer::start().await.unwrap();
    let attempts = usize::try_from(SETTINGS.max_retries + 2).unwrap_or(usize::MAX);
    enqueue_status(&server, 429, attempts);

    let error = status_of(&server).await;

    assert_eq!(
        server.requests().len(),
        attempts,
        "429 is a retry candidate"
    );
    assert_status(&error, 429);

    server.stop();
}

/// 500은 재시도 후보. 501은 후보가 아니라 시도를 한 번만 연다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_status_only_candidates_retries() {
    for (status, attempts) in [(500_u16, 3_usize), (501_u16, 1_usize)] {
        let server = MockServer::start().await.unwrap();
        enqueue_status(&server, status, attempts);

        let error = status_of(&server).await;

        assert_eq!(
            server.requests().len(),
            attempts,
            "status {status} runs {attempts} attempts"
        );
        assert_status(&error, status);

        server.stop();
    }
}

/// 상태 코드를 지정한 개수만큼 enqueue함.
/// 02-6 규칙은 최대 재시도 `retries`에 시도 `retries + 2`번까지 연다.
fn enqueue_status(server: &MockServer, status: u16, attempts: usize) {
    for _ in 0..attempts {
        server.enqueue(json_script(status, "server down"));
    }
}

/// 상태 오류의 Display·Debug에 키·호스트가 없고 가린 요약만 담긴다.
fn assert_status(error: &ModelsError, status: u16) {
    let ModelsError::Transport(TransportError::Status {
        status: code,
        summary,
        retry_after,
    }) = error
    else {
        panic!("test expects status error: {error}");
    };
    assert_eq!(*code, status, "status carries the code");
    assert!(retry_after.is_none(), "no header stays none");

    let display = error.to_string();
    let debug = format!("{error:?}");
    for secret in [KEY, "127.0.0.1"] {
        assert!(
            !display.contains(secret) && !debug.contains(secret),
            "status error hides host and key: {display}"
        );
    }
    assert!(
        !summary.expose().contains(KEY),
        "summary carries no key value"
    );
}
