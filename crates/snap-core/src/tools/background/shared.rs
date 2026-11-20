//! 백그라운드 목록을 여러 실행 판이 나누는 공유 창구

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use super::registry::BackgroundRegistry;

/// 백그라운드 목록을 여러 실행 판이 나누는 공유 창구
pub type SharedBackgroundRegistry = Arc<Mutex<BackgroundRegistry>>;

/// 백그라운드 목록 공유 창구 생성
#[must_use]
pub fn shared_background_registry() -> SharedBackgroundRegistry {
    Arc::new(Mutex::new(BackgroundRegistry::default()))
}

/// 공유 창구를 잠금. 잠금은 await 앞에 놓기
pub fn lock_background(shared: &SharedBackgroundRegistry) -> MutexGuard<'_, BackgroundRegistry> {
    shared
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
