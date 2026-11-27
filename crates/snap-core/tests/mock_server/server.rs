//! 모의 서버 실행 자리. 임시 포트 접속·응답 스크립트 적용·기록 저장 호출만 담당.
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use axum::Router;
use axum::extract::Request;
use axum::extract::State;
use axum::response::Response;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::connections::ClosedFlag;
use crate::connections::CountingListener;
use crate::recorder::record_request;
use crate::response::build_response;
use crate::script::MockScript;
use crate::script::StreamStop;
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

/// 핸들러가 공유하는 서버 상태. 응답 스크립트 큐·요청 기록·서버 끝낼 표시.
#[derive(Debug)]
struct ServerState {
    /// 응답 스크립트 큐.
    scripts: Mutex<VecDeque<MockScript>>,

    /// 받은 요청 기록.
    log: RequestLog,

    /// 서버를 끝낼 표시.
    shutdown: Mutex<Option<CancellationToken>>,
}

impl ServerState {
    /// 빈 상태.
    fn new() -> Self {
        Self {
            scripts: Mutex::new(VecDeque::new()),
            log: RequestLog::new(),
            shutdown: Mutex::new(None),
        }
    }

    /// 서버 끝낼 표시를 연결함. 끊기 규칙이 서버 연결을 정리하게 쓴다.
    fn attach_shutdown(&self, token: CancellationToken) {
        let Ok(mut shutdown) = self.shutdown.lock() else {
            return;
        };
        *shutdown = Some(token);
    }

    /// 서버를 끝낼 표시를 끊음. 끊기 규칙이 없으면 아무것도 하지 않음.
    fn cancel_shutdown(&self) {
        let Ok(shutdown) = self.shutdown.lock() else {
            return;
        };
        if let Some(token) = shutdown.as_ref() {
            token.cancel();
        }
    }

    /// 큐에 응답 스크립트를 추가함.
    fn push_script(&self, script: MockScript) {
        let Ok(mut scripts) = self.scripts.lock() else {
            return;
        };
        scripts.push_back(script);
    }

    /// 큐에서 응답 스크립트를 하나 뺌. 큐가 비면 404 응답.
    fn take_script(&self) -> MockScript {
        let Ok(mut scripts) = self.scripts.lock() else {
            return MockScript::with_status(404);
        };
        scripts
            .pop_front()
            .unwrap_or_else(|| MockScript::with_status(404))
    }
}

/// 요청을 받아 응답하는 라우터. 모든 경로를 같은 핸들러로 봄.
fn build_router(state: Arc<ServerState>) -> Router {
    Router::new().fallback(handle).with_state(state)
}

/// 요청을 기록하고 응답 스크립트대로 응답함. 끊기 규칙은 응답 뒤 서버를 끝낸다.
async fn handle(State(state): State<Arc<ServerState>>, request: Request) -> Response {
    let script = state.take_script();
    let recorded = record_request(request).await;
    state.log.push(recorded);
    sleep(script.header.wait()).await;
    let response = build_response(&script);
    if matches!(script.stop, StreamStop::Cut | StreamStop::Drop) {
        state.cancel_shutdown();
    }
    response
}
