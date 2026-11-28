//! 02-7 T3 `/v1` 보정 경로 시험. 요청 경로가 이중 `/v1/v1`이 아닌지만 봄(spec AC 6).

use snap_core::llm::ensure_v1;
use snap_core::llm::models_url;

use crate::materials::json_script;
use crate::materials::models_of;
use crate::materials::models_of_without_key;
use crate::server::MockServer;

/// `base_url`이 `/v1`로 끝나면 요청 경로는 `/v1/models` 한 번만 붙음.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_request_path_has_single_v1() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(json_script(200, r#"{"data":[{"id":"qwen-id"}]}"#));

    let ids = models_of(&server).await.expect("list works");

    assert_eq!(ids, vec!["qwen-id".to_owned()]);
    let recorded = &server.requests().snapshot()[0];
    assert_eq!(recorded.path, "/v1/models", "path has single v1");

    server.stop();
}

/// `base_url`에 `/v1`이 없으면 `/v1`을 한 번만 붙여 요청함.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_request_path_adds_single_v1_when_missing() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(json_script(200, r#"{"data":[{"id":"qwen-id"}]}"#));

    let ids = models_of_without_key(&server).await.expect("list works");

    assert_eq!(ids, vec!["qwen-id".to_owned()]);
    let recorded = &server.requests().snapshot()[0];
    assert_eq!(recorded.path, "/v1/models", "path adds single v1");

    server.stop();
}

/// `ensure_v1`은 `/v1`이 있을 때 원문 그대로, 없을 때 한 번만 붙임.
#[test]
fn ensure_v1_keeps_single_version() {
    let with_version = "http://127.0.0.1:1234/v1";
    let without_version = "http://127.0.0.1:1234";

    assert_eq!(ensure_v1(with_version), with_version, "keeps existing v1");
    assert_eq!(
        ensure_v1(without_version),
        "http://127.0.0.1:1234/v1",
        "adds v1 once"
    );
}

/// `models_url`은 `/v1` 유무와 상관없이 `/v1/models` 한 번만 붙임.
#[test]
fn models_url_has_single_v1_models() {
    assert_eq!(
        models_url("http://127.0.0.1:1234/v1"),
        "http://127.0.0.1:1234/v1/models"
    );
    assert_eq!(
        models_url("http://127.0.0.1:1234"),
        "http://127.0.0.1:1234/v1/models"
    );
}
