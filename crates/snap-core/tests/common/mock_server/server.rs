//! 모의 서버의 수명(띄우기·접기)과 시험 조종 창구만 다룹니다.
//!
//! 응답 스크립트 규격은 `super::script`, 공유 상태는 `super::shared`, 연결 수 가드는
//! `super::guard`, 본문 스트림 구현은 `super::body_stream`, 스크립트 대기열 규칙은
//! `super::queue`에 있습니다(서버 수명 책임과 스크립트 대기열 책임 분리).

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::routing::get;
use axum::routing::post;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use super::chat_handler::handle_chat_completions;
use super::models_handler::handle_models;
use super::queue;
use super::record::RecordedRequest;
use super::script::MockResponseScript;
use super::shared::Shared;

/// 로컬에서 뜨는 모의 서버입니다. 채팅 완료(`/chat/completions`)와 모델 목록(`/models`) 경로를 함께 다룹니다.
pub struct MockServer {
    base_url: String,
    shared: Arc<Shared>,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    join: Option<JoinHandle<()>>,
}

impl MockServer {
    /// `127.0.0.1`의 빈 포트에 모의 서버를 띄웁니다. 채팅 완료와 모델 목록 경로를 함께 등록합니다.
    pub async fn spawn(script: MockResponseScript) -> Self {
        Self::spawn_with(script, |shared| {
            Router::new()
                .route("/chat/completions", post(handle_chat_completions))
                .route("/models", get(handle_models))
                .with_state(shared)
        })
        .await
    }

    /// 왕복 여러 턴(에이전트 루프처럼 한 요청이 아닌 요청) 시험을 위해, 스크립트를 앞부터
    /// 순서대로 써서 응답하는 서버를 띄웁니다. 대기열이 비면 마지막 스크립트를 그대로 씁니다.
    /// 대기열 채우기 규칙은 `super::queue`에 있습니다(서버 수명 책임과 스크립트 대기열 책임 분리).
    pub async fn spawn_queue(scripts: Vec<MockResponseScript>) -> Self {
        let (tail, remaining) = queue::split(scripts);
        // 서버가 요청받기 전에 대기열을 채운다. 띄운 뒤 채우면 첫 요청이 뒤 스크립트로 답할 수 있다(queue 경쟁 방지).
        let shared = Arc::new(Shared::new(tail));
        queue::fill(&shared, remaining).await;
        Self::spawn_shared(shared, |shared| {
            Router::new()
                .route("/chat/completions", post(handle_chat_completions))
                .route("/models", get(handle_models))
                .with_state(shared)
        })
        .await
    }

    /// 라우트를 직접 정해서 띄우는 기본 만들기입니다. 라우트 등록 쪽만 바뀌면 서버 수명 코드를
    /// 다시 적지 않습니다. `routes`는 서버가 쓸 공유 상태를 라우트에 붙인 값을 받습니다.
    async fn spawn_with(
        script: MockResponseScript,
        routes: impl FnOnce(Arc<Shared>) -> Router,
    ) -> Self {
        Self::spawn_shared(Arc::new(Shared::new(script)), routes).await
    }

    /// 공유 상태를 받아 라우트를 붙이고 포트에 뜨는 실제 만들기입니다. 대기열을 미리 채운
    /// 상태를 넘길 수 있게 나눈 것입니다(`spawn_queue`의 경쟁 방지).
    async fn spawn_shared(shared: Arc<Shared>, routes: impl FnOnce(Arc<Shared>) -> Router) -> Self {
        let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0)))
            .await
            .expect("모의 서버 포트 바인딩");
        let addr = listener.local_addr().expect("모의 서버 로컬 주소");
        let app = routes(shared.clone());

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

    /// `{base_url}/models` 경로입니다. 모델 목록 조회 시험이 씁니다.
    pub fn models_url(&self) -> String {
        format!("{}/models", self.base_url())
    }

    /// 모델 목록 라우트만 띄우는 서버입니다. `/models`와 `/v1`이 base_url에 붙어 있는 주소도 함께
    /// 등록해서, base_url에 `/v1`이 있어도 경로가 `/v1/v1`로 이중으로 결합되지 않음을 보는 시험이 씁니다.
    pub async fn spawn_models_only(script: MockResponseScript) -> Self {
        Self::spawn_with(script, |shared| {
            Router::new()
                .route("/models", get(handle_models))
                .route("/v1/models", get(handle_models))
                .with_state(shared)
        })
        .await
    }

    /// `/models` 라우트 하나만 띄우고 스크립트 시점(헤더 지연·행·본문 조각 지연)을 그대로 따르는 서버입니다.
    ///
    /// 취소·시간 초과 시험이 채팅과 같은 스크립트 규격을 목록 전용 경로에서 그대로 쓰려고 엽니다.
    pub async fn spawn_models_script(script: MockResponseScript) -> Self {
        Self::spawn_with(script, |shared| {
            Router::new()
                .route("/models", get(handle_models))
                .with_state(shared)
        })
        .await
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
        self.shared.open_connections()
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
