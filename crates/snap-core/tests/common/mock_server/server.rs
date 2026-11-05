#![allow(dead_code, unreachable_pub)]

//! 모의 서버의 수명(띄우기·접기)과 요청 기록·연결 수 조종입니다.
//!
//! 응답 스크립트 규격은 `super::script`, 본문 스트림 구현은 `super::handler`에 있습니다.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, Notify};
use tokio::task::JoinHandle;

use super::handler::handle_chat_completions;
use super::script::{MockResponseScript, RecordedRequest};

/// 핸들러·가드가 공유하는 상태입니다. 필트는 이 파일의 조종 함수만 써서 다룹니다.
pub struct Shared {
    script: Mutex<MockResponseScript>,
    recorded: Mutex<Vec<RecordedRequest>>,
    open_connections: AtomicUsize,
    connection_change: Notify,
}

/// 로컬에서 뜨는 모의 채팅 완료 서버입니다.
pub struct MockServer {
    base_url: String,
    shared: Arc<Shared>,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    join: Option<JoinHandle<()>>,
}

impl MockServer {
    /// `127.0.0.1`의 빈 포트에 모의 서버를 띄웁니다.
    pub async fn spawn(script: MockResponseScript) -> Self {
        let shared = Arc::new(Shared {
            script: Mutex::new(script),
            recorded: Mutex::new(Vec::new()),
            open_connections: AtomicUsize::new(0),
            connection_change: Notify::new(),
        });

        let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0)))
            .await
            .expect("모의 서버 포트 바인딩");
        let addr = listener.local_addr().expect("모의 서버 로컬 주소");
        let app = Router::new()
            .route("/chat/completions", post(handle_chat_completions))
            .with_state(shared.clone());

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        let join = tokio::spawn(async move {
            let serve = axum::serve(listener, app).with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            });
            let _ = serve.await;
        });

        Self {
            base_url: format!("http://{addr}"),
            shared,
            shutdown: Some(shutdown_tx),
            join: Some(join),
        }
    }

    /// `http://127.0.0.1:포트` 형태의 주소입니다.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// `{base_url}/chat/completions` 경로입니다.
    pub fn chat_completions_url(&self) -> String {
        format!("{}/chat/completions", self.base_url())
    }

    /// 다음 요청부터 쓸 응답 스크립트를 바꿉니다.
    pub async fn set_script(&self, script: MockResponseScript) {
        *self.shared.script.lock().await = script;
    }

    /// 지금까지 기록한 요청 목록을 복사해 돌려줍니다.
    pub async fn recorded_requests(&self) -> Vec<RecordedRequest> {
        self.shared.recorded.lock().await.clone()
    }

    /// 지금 열려 있는 연결 수입니다.
    pub fn open_connections(&self) -> usize {
        self.shared.open_connections.load(Ordering::SeqCst)
    }

    /// 열린 연결이 0이 될 때까지 기다립니다. 시간 안에 되면 true입니다.
    pub async fn wait_until_idle(&self, timeout: Duration) -> bool {
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            if self.open_connections() == 0 {
                return true;
            }
            let now = tokio::time::Instant::now();
            if now >= deadline {
                return self.open_connections() == 0;
            }
            let remaining = deadline - now;
            let notified = self.shared.connection_change.notified();
            if self.open_connections() == 0 {
                return true;
            }
            tokio::select! {
                _ = notified => {}
                _ = tokio::time::sleep(remaining) => {
                    return self.open_connections() == 0;
                }
            }
        }
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(join) = self.join.take() {
            join.abort();
        }
    }
}

/// 핸들러가 현재 응답 스크립트 사본을 꺼냅니다.
pub async fn current_script(shared: &Arc<Shared>) -> MockResponseScript {
    shared.script.lock().await.clone()
}

/// 핸들러가 받은 요청을 기록에 남깁니다.
pub async fn record(shared: &Arc<Shared>, request: RecordedRequest) {
    shared.recorded.lock().await.push(request);
}

/// 연결 수를 세는 가드입니다. 만들 때 +1, 버릴 때 -1 합니다.
pub struct ConnectionGuard {
    shared: Arc<Shared>,
}

impl ConnectionGuard {
    pub fn new(shared: &Arc<Shared>) -> Self {
        shared.open_connections.fetch_add(1, Ordering::SeqCst);
        shared.connection_change.notify_waiters();
        Self {
            shared: shared.clone(),
        }
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.shared.open_connections.fetch_sub(1, Ordering::SeqCst);
        self.shared.connection_change.notify_waiters();
    }
}
