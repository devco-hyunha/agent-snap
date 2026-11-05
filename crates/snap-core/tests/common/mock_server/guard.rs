//! 열려 있는 연결 수를 세는 가드 하나만 다룹니다.
//!
//! 만들 때 +1, 버릴 때 -1 하고, 열린 연결 수가 바뀌면 시험에 알린다.
//! 모의 서버 수명(`server`)·공유 상태(`shared`)와 파일을 나눴다(SRP, dev-principles 2절).

use std::sync::Arc;
use std::sync::atomic::Ordering;

use super::shared::Shared;

/// 연결 수를 세는 가드입니다. 만들 때 +1, 버릴 때 -1 합니다.
pub struct ConnectionGuard {
    shared: Arc<Shared>,
}

impl ConnectionGuard {
    /// 가드를 만들고 공유 상태의 연결 수를 한 올립니다.
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
