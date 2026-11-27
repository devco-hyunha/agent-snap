//! 02-7 T3 목록 GET 요청 헤더 시험. 키가 있을 때만 Bearer를 보내는지만 봄.

use snap_core::config::SecretString;

use crate::materials::KEY;
use crate::materials::base_url;
use crate::materials::json_script;
use crate::materials::models_of_keyed;
use crate::materials::models_of_without_key;
use crate::server::MockServer;

/// 키 없이 조회하면 인증 헤더를 보내지 않는다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_request_without_key_sends_no_authorization() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(json_script(200, r#"{"data":[{"id":"qwen-id"}]}"#));

    let ids = models_of_without_key(&server)
        .await
        .expect("keyless list works");

    assert_eq!(ids, vec!["qwen-id".to_owned()]);
    let recorded = &server.requests().snapshot()[0];
    assert_eq!(recorded.method, "GET", "list uses GET");
    assert!(
        recorded.header("authorization").is_none(),
        "keyless list sends no authorization header"
    );

    server.stop();
}

/// 키가 있으면 Bearer 헤더로만 보낸다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_request_with_key_sends_bearer_header() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(json_script(200, r#"{"data":[{"id":"qwen-id"}]}"#));

    let key = SecretString::new(KEY.to_owned());
    let ids = models_of_keyed(&base_url(&server), Some(&key))
        .await
        .expect("keyed list works");

    assert_eq!(ids, vec!["qwen-id".to_owned()]);
    let recorded = &server.requests().snapshot()[0];
    assert_eq!(
        recorded.header("authorization"),
        Some("Bearer sk-modelsvalue123"),
        "key goes to authorization header only"
    );

    server.stop();
}
