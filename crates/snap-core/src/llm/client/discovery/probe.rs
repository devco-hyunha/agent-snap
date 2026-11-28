//! 자동 발견 프로브 자리. 연결 한 번·대상 판단·주소 채우기만 담음.
//!
//! 상시 폴링·재시도·캐시·mDNS·`/health` 먼저 확인은 두지 않는다.
//! reqwest 타입은 밖에 내지 않는다.

use tokio_util::sync::CancellationToken;

use crate::config::secret_string::SecretString;
use crate::consts::defaults::DEFAULT_LMSTUDIO_BASE_URL;
use crate::consts::defaults::DEFAULT_OLLAMA_BASE_URL;
use crate::llm::client::discovery::outcome::DiscoveryOutcome;
use crate::llm::client::kind::EndpointKind;
use crate::llm::client::open::build_client;
use crate::llm::client::timings::TransportTimings;

/// 종류에 대응하는 기본 주소. 대상 종류가 아니면 없음.
pub const fn default_base_url(kind: EndpointKind) -> Option<&'static str> {
    match kind {
        EndpointKind::Ollama => Some(DEFAULT_OLLAMA_BASE_URL),
        EndpointKind::Lmstudio => Some(DEFAULT_LMSTUDIO_BASE_URL),
        _ => None,
    }
}

/// 기본 주소 값을 받아 자동 발견함. 시험이 루프백 주소를 넣는 자리.
///
/// `base_url`이 비어 있으면 `default_url`에만 연결을 한 번 시도한다.
/// 주소가 채워진 경우·연결 실패는 발견 실패로 본다.
pub async fn discover_from(
    kind: EndpointKind,
    base_url: &str,
    default_url: &str,
    timings: &TransportTimings,
    cancel: &CancellationToken,
) -> DiscoveryOutcome {
    let is_target = default_base_url(kind).is_some();
    if !is_target || !base_url.trim().is_empty() {
        return not_found(kind);
    }
    if probe_reachable(default_url, timings, cancel).await {
        return DiscoveryOutcome::Found {
            base_url: SecretString::new(default_url.to_owned()),
        };
    }
    not_found(kind)
}

/// 주소 한곳에 연결이 열리는지 한 번만 시도함. 응답 헤더가 오면 참.
///
/// 연결이 안 되면 참이 아니다. 재시도·폴링은 하지 않는다.
async fn probe_reachable(
    base_url: &str,
    timings: &TransportTimings,
    cancel: &CancellationToken,
) -> bool {
    let Ok(client) = build_client(timings) else {
        return false;
    };
    let builder = client.get(base_url);
    let sent = tokio::select! {
        () = cancel.cancelled() => return false,
        sent = tokio::time::timeout(timings.connect, builder.send()) => sent,
    };
    matches!(sent, Ok(Ok(_)))
}

/// 발견 실패 결과.
const fn not_found(kind: EndpointKind) -> DiscoveryOutcome {
    DiscoveryOutcome::NotFound { kind }
}
