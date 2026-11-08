//! 모의 서버 핸들러가 공유하는 상태와 그 조종 만들기, 연결 수 가드, 응답 시작 조립을 다룹니다.
//!
//! 스크립트 규격(`script`)·서버 수명(`server`)·요청 기록(`record`)과 파일을 나눴고,
//! 이 파일은 핸들러와 가드가 쓰는 공유 상태의 정본입니다.
//! 핸들러가 같은 조립을 다시 적지 않게 응답 시작·헤더 대기 조립도 여기에 뒀습니다(DRY).
//! 본문 스트림과 목록 본문 조립은 각각 자기 파일의 책임입니다(SRP).

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::{Mutex, Notify};

use super::record::RecordedRequest;
use super::script::MockResponseScript;

/// 핸들러·가드가 공유하는 상태입니다. 필드는 이 파일의 조종 함수만 써서 다룹니다.
pub struct Shared {
    pub(super) script: Mutex<MockResponseScript>,
    /// 왕복 여러 턴 시험이 쓸 스크립트 대기열입니다. queue에서부터 앞을 먼저 꺼냅니다.
    pub(super) script_queue: Mutex<VecDeque<MockResponseScript>>,
    pub(super) recorded: Mutex<Vec<RecordedRequest>>,
    pub(super) open_connections: AtomicUsize,
    pub(super) connection_change: Notify,
}

impl Shared {
    /// 서버가 띄워질 때 한 번 만든다.
    pub(super) fn new(script: MockResponseScript) -> Self {
        Self {
            script: Mutex::new(script),
            script_queue: Mutex::new(VecDeque::new()),
            recorded: Mutex::new(Vec::new()),
            open_connections: AtomicUsize::new(0),
            connection_change: Notify::new(),
        }
    }

    /// 지금 열려 있는 연결 수입니다.
    pub(super) fn open_connections(&self) -> usize {
        self.open_connections.load(Ordering::SeqCst)
    }
}

/// 핸들러가 현재 응답 스크립트 사본을 꺼냅니다.
///
/// 대기열에 스크립트가 남아 있으면 앞의 것을 꺼내 쓰고, 없으면 기본 스크립트를 그대로 씁니다.
/// 왕복 여러 턴 시험은 `queue`에 스크립트를 채워 턴마다 다른 응답을 받을 수 있습니다.
pub(super) async fn current_script(shared: &Arc<Shared>) -> MockResponseScript {
    if let Some(script) = shared.script_queue.lock().await.pop_front() {
        return script;
    }
    shared.script.lock().await.clone()
}

/// 핸들러가 받은 요청을 기록에 남깁니다.
pub(super) async fn record(shared: &Arc<Shared>, request: RecordedRequest) {
    shared.recorded.lock().await.push(request);
}
