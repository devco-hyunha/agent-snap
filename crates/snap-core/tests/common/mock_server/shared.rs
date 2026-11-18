//! 모의 서버 핸들러가 공유하는 상태와 그 조종 만들기
//!
//! 핸들러와 가드가 쓰는 공유 상태의 정본

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::{Mutex, Notify};

use super::record::RecordedRequest;
use super::script::MockResponseScript;

/// 핸들러·가드가 공유하는 상태. 필드는 이 파일의 조종 함수만 씀
pub struct Shared {
    pub(super) script: Mutex<MockResponseScript>,
    /// 왕복 여러 턴 시험이 쓸 스크립트 대기열. 앞을 먼저 꺼냄
    pub(super) script_queue: Mutex<VecDeque<MockResponseScript>>,
    pub(super) recorded: Mutex<Vec<RecordedRequest>>,
    pub(super) open_connections: AtomicUsize,
    pub(super) connection_change: Notify,
}

impl Shared {
    /// 서버가 띄워질 때 한 번 만듦
    pub(super) fn new(script: MockResponseScript) -> Self {
        Self {
            script: Mutex::new(script),
            script_queue: Mutex::new(VecDeque::new()),
            recorded: Mutex::new(Vec::new()),
            open_connections: AtomicUsize::new(0),
            connection_change: Notify::new(),
        }
    }

    /// 지금 열려 있는 연결 수
    pub(super) fn open_connections(&self) -> usize {
        self.open_connections.load(Ordering::SeqCst)
    }
}

/// 핸들러가 현재 응답 스크립트 사본을 꺼냄
pub(super) async fn current_script(shared: &Arc<Shared>) -> MockResponseScript {
    if let Some(script) = shared.script_queue.lock().await.pop_front() {
        return script;
    }
    shared.script.lock().await.clone()
}

/// 핸들러가 받은 요청을 기록에 남김
pub(super) async fn record(shared: &Arc<Shared>, request: RecordedRequest) {
    shared.recorded.lock().await.push(request);
}
