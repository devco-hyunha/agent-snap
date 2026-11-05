//! 모의 서버 핸들러가 공유하는 상태와 그 조종 만들기, 연결 수 가드, 응답 시작 조립을 다룹니다.
//!
//! 스크립트 규격(`script`)·서버 수명(`server`)·요청 기록(`record`)과 파일을 나눴고,
//! 이 파일은 핸들러와 가드가 쓰는 공유 상태의 정본입니다.
//! 핸들러가 같은 조립을 다시 적지 않게 응답 시작·헤더 대기 조립도 여기에 뒀습니다(DRY).
//! 본문 스트림과 목록 본문 조립은 각각 자기 파일의 책임입니다(SRP).

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::{Mutex, Notify};

use super::record::RecordedRequest;
use super::script::MockResponseScript;

/// 핸들러·가드가 공유하는 상태입니다. 필드는 이 파일의 조종 함수만 써서 다룹니다.
pub struct Shared {
    pub(super) script: Mutex<MockResponseScript>,
    pub(super) recorded: Mutex<Vec<RecordedRequest>>,
    pub(super) open_connections: AtomicUsize,
    pub(super) connection_change: Notify,
}

impl Shared {
    /// 서버가 띄워질 때 한 번 만든다.
    pub(super) fn new(script: MockResponseScript) -> Self {
        Self {
            script: Mutex::new(script),
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
pub(super) async fn current_script(shared: &Arc<Shared>) -> MockResponseScript {
    shared.script.lock().await.clone()
}

/// 핸들러가 받은 요청을 기록에 남깁니다.
pub(super) async fn record(shared: &Arc<Shared>, request: RecordedRequest) {
    shared.recorded.lock().await.push(request);
}
