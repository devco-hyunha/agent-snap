//! 02-7 T4 연결 실패 시험. 연결이 안 되는 주소에서 발견 실패로 끝나고 프로브가 한 번만 나가는지만 봄(spec AC 7).

use snap_core::llm::DiscoveryOutcome;
use snap_core::llm::EndpointKind;
use tokio::net::TcpListener;

use crate::materials::closed_loopback_url;
use crate::materials::discover_with_default;
use crate::materials::loopback_url;

/// 연결이 거부되는 주소에서는 발견 실패로 끝난다. 오류가 아니다.
#[tokio::test]
async fn discovery_reports_not_found_when_connection_refused() {
    let default_url = closed_loopback_url().await;

    let outcome = discover_with_default(EndpointKind::Ollama, "", &default_url).await;

    assert!(
        outcome.is_not_found(),
        "refused connection is a discovery failure, not an error"
    );
    assert_eq!(
        outcome,
        DiscoveryOutcome::NotFound {
            kind: EndpointKind::Ollama
        },
        "failure carries the target kind only"
    );
    assert_eq!(outcome.filled(), None, "failure fills nothing");
}

/// 연결이 끊기는 주소에서도 발견 실패이고, 연결 시도는 한 번만 나간다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn discovery_failure_probes_once() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let default_url = loopback_url(address);

    let accepted = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        drop(stream);
        1u32
    });

    let outcome = discover_with_default(EndpointKind::Lmstudio, "", &default_url).await;

    assert!(outcome.is_not_found(), "closed connection is not found");
    let attempts = accepted.await.expect("test accepts one connection");
    assert_eq!(attempts, 1, "probe opens one connection only");
}
