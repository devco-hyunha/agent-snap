//! 02-7 T4 자동 발견 시험 공통 재료. 시간 값·루프백 주소·발견 창구·목록 조회 창구만 담음.
//! 같은 시험 묶음의 채움·실패·대상·목록 연결 시험이 재사용하는 재료 파일.

use std::net::SocketAddr;
use std::time::Duration;

use snap_core::config::SecretString;
use snap_core::llm::DiscoveryOutcome;
use snap_core::llm::EndpointKind;
use snap_core::llm::ModelsError;
use snap_core::llm::RetrySettings;
use snap_core::llm::SystemRandomSource;
use snap_core::llm::TransportTimings;
use snap_core::llm::discover_from;
use snap_core::llm::list_models;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;

/// 시험용 시간 값. 프로브 연결만 짧게 재고, 헤더 대기는 프로브가 쓰지 않음.
pub(crate) const TIMINGS: TransportTimings = TransportTimings {
    connect: Duration::from_secs(2),
    header_wait: Duration::from_secs(2),
    stream_idle: Duration::from_secs(2),
};

/// 기본 주소 값을 주입해 자동 발견함. 상용 포트 상수는 시험에서 루프백 주소로 채운다.
pub(crate) async fn discover_with_default(
    kind: EndpointKind,
    base_url: &str,
    default_url: &str,
) -> DiscoveryOutcome {
    let cancel = CancellationToken::new();
    discover_from(kind, base_url, default_url, &TIMINGS, &cancel).await
}

/// 모의 서버 주소를 자동 발견 기본 주소 형태(`/v1` 포함)로 만듦.
pub(crate) fn server_default_url(server: &MockServer) -> String {
    format!("{}/v1", server.base_url())
}

/// 연결하면 거부되는 루프백 주소. 리스너를 붙였다 내려서 프로브 실패 재료로 쓴다.
#[allow(clippy::unwrap_used)]
pub(crate) async fn closed_loopback_url() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    loopback_url(address)
}

/// 루프백 소켓 주소를 프로브가 쓰는 기본 주소 형태로 만듦.
pub(crate) fn loopback_url(address: SocketAddr) -> String {
    format!("http://{address}/v1")
}

/// 시험용 재시도 설정. 재시도 대기는 0, 추가 시도는 한 번만.
pub(crate) const MODELS_SETTINGS: RetrySettings = RetrySettings {
    max_retries: 1,
    backoff_min: Duration::ZERO,
    backoff_max: Duration::from_millis(1),
    retry_after_max: 60,
};

/// JSON 본문을 chunked 조각으로 보내고 끝 표시로 닫는 응답 스크립트.
pub(crate) fn json_script(status: u16, body: &str) -> MockScript {
    MockScript::with_status(status).with_body(vec![
        StreamStep::chunk(body.to_owned()),
        StreamStep::Stop(crate::script::StreamStop::End),
    ])
}

/// 채운 주소로 모델 목록을 읽어서 결과를 그대로 돌려줌. 키는 붙이지 않음.
pub(crate) async fn models_of_filled(base_url: &str) -> Result<Vec<String>, ModelsError> {
    models_of_keyed(base_url, None).await
}

/// 조회 창구 한곳. 키 유무를 그대로 넘겨 키 있음·키 없음 경로를 재료에서 함께 볼 수 있게 함.
pub(crate) async fn models_of_keyed(
    url: &str,
    key: Option<&SecretString>,
) -> Result<Vec<String>, ModelsError> {
    list_models(
        url,
        key,
        &TIMINGS,
        &MODELS_SETTINGS,
        SystemRandomSource::default(),
        &CancellationToken::new(),
    )
    .await
}
