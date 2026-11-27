//! 02-7 T4 채운 주소 목록 연결 시험. 자동 발견이 채운 주소로 목록 조회가 이어지는지만 봄(spec AC 7).

use snap_core::llm::EndpointKind;

use crate::materials::discover_with_default;
use crate::materials::json_script;
use crate::materials::models_of_filled;
use crate::materials::server_default_url;
use crate::server::MockServer;

/// `ollama`이고 `base_url`이 비면 채워진 주소로 목록 조회가 이어진다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn filled_address_lists_models() {
    let server = MockServer::start().await.unwrap();
    let default_url = server_default_url(&server);
    server.enqueue(json_script(200, "{}"));
    server.enqueue(json_script(200, r#"{"data":[{"id":"qwen-id"}]}"#));

    let outcome = discover_with_default(EndpointKind::Ollama, "", &default_url).await;
    let filled = outcome.filled().expect("discovery fills the address");
    let ids = models_of_filled(filled).await.expect("list works");

    assert_eq!(
        ids,
        vec!["qwen-id".to_owned()],
        "filled address lists models"
    );
    let requests = server.requests().snapshot();
    assert_eq!(requests.len(), 2, "probe request then list request");
    assert_eq!(
        requests[1].path, "/v1/models",
        "list reads the filled address path"
    );

    server.stop();
}

/// `lmstudio`도 같은 규칙으로 채운 주소의 목록을 읽는다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn lmstudio_filled_address_lists_models() {
    let server = MockServer::start().await.unwrap();
    let default_url = server_default_url(&server);
    server.enqueue(json_script(200, "{}"));
    server.enqueue(json_script(200, r#"{"data":[{"id":"llama-id"}]}"#));

    let outcome = discover_with_default(EndpointKind::Lmstudio, "", &default_url).await;
    let filled = outcome.filled().expect("discovery fills the address");
    let ids = models_of_filled(filled).await.expect("list works");

    assert_eq!(ids, vec!["llama-id".to_owned()], "lmstudio lists models");

    server.stop();
}
