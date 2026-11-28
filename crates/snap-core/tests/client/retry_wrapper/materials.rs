//! 02-4 재시도 래퍼 시험 재료 자리. 래퍼 조립·난수원·시간 값·서버 조립 재료만 담음.
//! 같은 시험 묶음의 경계·포기·취소·가리기 시험이 재사용하는 재료 파일.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::RandomSource;
use snap_core::llm::RetryClient;
use snap_core::llm::RetrySettings;
use tokio::net::TcpListener;

use crate::script::ABORT_GAP;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;
use crate::stream_open::base_url;
use crate::stream_open::transport_timings;
use crate::stream_payload::FIRST_CHUNK;

/// 시험용 연결 시간. 연결 단계 실패가 시간 초과로 잡히지 않게 넉넉하게 둔다.
pub(crate) const CONNECT: Duration = Duration::from_secs(5);

/// 시험용 헤더 대기 시간.
pub(crate) const HEADER_WAIT: Duration = Duration::from_secs(2);

/// 시험용 스트림 유휴 시간.
pub(crate) const IDLE: Duration = Duration::from_millis(120);

/// 최대 재시도 2회는 시도 4까지 연다. 02-4의 `max_retries + 1` 셈을 그대로 따른다.
pub(crate) const LIMITED_ATTEMPTS: usize = 4;

/// 정상 스트림을 연 요청 한 번.
pub(crate) const ONE_REQUEST: usize = 1;

/// 재시도 뒤에 연 요청까지 합친 수.
pub(crate) const TWO_REQUESTS: usize = 2;

/// 시험용 재시도 설정. 대기 최소는 0이라 백오프 대기가 시험을 늘리지 않는다.
pub(crate) const SETTINGS: RetrySettings = RetrySettings {
    max_retries: 2,
    backoff_min: Duration::ZERO,
    backoff_max: Duration::from_secs(1),
    retry_after_max: 60,
};

/// 항상 0을 뽑는 난수원. 백오프 대기를 0으로 세워 시험을 빠르게 세운다.
#[derive(Clone, Copy)]
pub(crate) struct ZeroSource;

impl RandomSource for ZeroSource {
    fn next_inclusive(&mut self, _max: u64) -> u64 {
        0
    }
}

/// 모의 서버로 향하는 재시도 래퍼.
pub(crate) fn wrapper(
    server: &MockServer,
    settings: RetrySettings,
) -> RetryClient<HttpTransport, ZeroSource> {
    let transport = HttpTransport::with_timings(
        &base_url(server),
        EndpointKind::Openai,
        None,
        transport_timings(CONNECT, HEADER_WAIT, IDLE),
    );
    RetryClient::new(transport, settings, ZeroSource)
}

/// 연결을 받아 즉시 끊는 서버. 연결을 받은 횟수를 센다.
pub(crate) async fn dropped_connection_server() -> (SocketAddr, Arc<AtomicUsize>) {
    #[allow(clippy::expect_used)]
    {
        let any_port: SocketAddr = "127.0.0.1:0".parse().expect("test address");
        #[allow(clippy::unwrap_used)]
        let listener = TcpListener::bind(any_port).await.unwrap();
        let address = listener.local_addr().expect("listener address");
        let accepted = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&accepted);
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    return;
                };
                counter.fetch_add(1, Ordering::Relaxed);
                drop(stream);
            }
        });
        (address, accepted)
    }
}

/// 헤더를 받은 뒤 본문을 급히 끊는 스크립트. 본문 끝 표시를 보내지 않는다.
pub(crate) fn abort_after_first_chunk() -> MockScript {
    MockScript::ok()
        .with_body(vec![
            StreamStep::chunk(FIRST_CHUNK).chunk_after(ABORT_GAP),
            StreamStep::Stop(StreamStop::Abort),
        ])
        .with_stop(StreamStop::Abort)
}
