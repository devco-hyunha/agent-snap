//! 02-7 T4 채움 성공 시험. 루프백에 서버가 서 있을 때 기본 주소가 채워지는지만 봄(spec AC 7).

use snap_core::config::SecretString;
use snap_core::llm::EndpointKind;

use crate::materials::discover_with_default;
use crate::materials::server_default_url;
use crate::server::MockServer;

/// `ollama`이고 `base_url`이 비면 연결이 열리는 기본 주소로 채운다.
#[tokio::test]
async fn ollama_fills_default_when_server_reachable() {
    let server = MockServer::start().await.expect("server starts");
    let default_url = server_default_url(&server);

    let outcome = discover_with_default(EndpointKind::Ollama, "", &default_url).await;

    assert_eq!(
        outcome,
        snap_core::llm::DiscoveryOutcome::Found {
            base_url: SecretString::new(default_url.clone())
        },
        "reachable loopback fills the address"
    );
    assert_eq!(outcome.filled(), Some(default_url.as_str()));

    server.stop();
}

/// `lmstudio`도 같은 규칙으로 채운다.
#[tokio::test]
async fn lmstudio_fills_default_when_server_reachable() {
    let server = MockServer::start().await.expect("server starts");
    let default_url = server_default_url(&server);

    let outcome = discover_with_default(EndpointKind::Lmstudio, "", &default_url).await;

    assert_eq!(
        outcome.filled(),
        Some(default_url.as_str()),
        "lmstudio fills the same shape"
    );

    server.stop();
}

/// 채운 주소로 프로브 요청이 한 번만 나간다.
#[tokio::test]
async fn discovery_probes_exactly_once() {
    let server = MockServer::start().await.expect("server starts");
    let default_url = server_default_url(&server);

    let _ = discover_with_default(EndpointKind::Ollama, "", &default_url).await;

    let requests = server.requests().snapshot();
    assert_eq!(requests.len(), 1, "probe sends one request only");

    server.stop();
}
