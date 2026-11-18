//! 열려 있는 연결 수를 세는 가드
//!
//! 만들 때 +1, 버릴 때 -1 하고, 열린 연결 수가 바뀌면 시험에 알림

use std::sync::Arc;
use std::sync::atomic::Ordering;

use super::shared::Shared;

/// 연결 수 세는 가드
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
