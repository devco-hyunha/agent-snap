//! 모의 서버 실행 자리. 임시 포트 접속·스크립트 큐·서버 끝내기만 담당.
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::connections::ClosedFlag;
use crate::connections::CountingListener;
use crate::router::build_router;
use crate::script::MockScript;
use crate::state::ServerState;
use crate::types::RequestLog;

/// 로컬 임시 포트에 붙은 시험용 모의 서버. 응답 스크립트와 요청 기록을 함께 쓴다.
#[derive(Debug)]
pub struct MockServer {
    /// 서버가 붙은 주소.
    address: SocketAddr,

    /// 받은 요청 기록.
    log: RequestLog,

    /// 응답 스크립트 큐를 핸들러와 공유하는 서버 상태.
    state: Arc<ServerState>,

    /// 서버를 끝낼 표시.
    shutdown: CancellationToken,

    /// 서버 쪽 연결이 끝났는지 확인하는 표시.
    closed: ClosedFlag,
}

impl MockServer {
    /// 로컬 임시 포트에 모의 서버를 띄움.
    pub async fn start() -> std::io::Result<Self> {
        let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await?;
        let address = listener.local_addr()?;
        let closed = ClosedFlag::new();
        let connections = CountingListener::new(listener, &closed);
        let state = Arc::new(ServerState::new());
        let shutdown = CancellationToken::new();
        state.attach_shutdown(shutdown.clone());
        let signal = shutdown.clone().cancelled_owned();
        let router = build_router(Arc::clone(&state));
        let serve = axum::serve(connections, router).with_graceful_shutdown(signal);
        tokio::spawn(async move {
            let _ = serve.await;
        });
        Ok(Self {
            address,
            log: state.log.clone(),
            state,
            shutdown,
            closed,
        })
    }

    /// 서버가 붙은 주소.
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// 요청을 보낼 기본 주소. 끝에 `/`가 없다.
    pub fn base_url(&self) -> String {
        format!("http://{}", self.address)
    }

    /// 지금까지 받은 요청 기록.
    pub fn requests(&self) -> &RequestLog {
        &self.log
    }

    /// 응답 스크립트를 순서대로 내보내는 큐에 추가함.
    pub fn enqueue(&self, script: MockScript) {
        self.state.push_script(script);
    }

    /// 서버 쪽 연결이 끝났는지.
    pub fn is_connection_closed(&self) -> bool {
        self.closed.is_closed()
    }

    /// 서버를 끝냄.
    pub fn stop(&self) {
        self.shutdown.cancel();
    }
}
