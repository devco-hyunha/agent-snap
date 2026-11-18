//! `ToolContext`의 승인 표식·취소 토큰 관리 자리

use std::sync::atomic::Ordering;

use tokio_util::sync::CancellationToken;

use crate::tools::tool::ToolContext;

impl ToolContext {
    /// 승인 흐름이 "이 호출은 승인받았다" 표식을 채우는 자리
    pub fn mark_approved(&self) {
        self.shared.approved.store(true, Ordering::Relaxed);
    }

    /// 승인 표식을 확인해 승인된 호출이면 소비
    ///
    /// 확인하는 자마다 표식을 소비하므로 승인자 응답이 승인한 재호출 한 개에만 쓰임
    pub fn take_approved(&self) -> bool {
        self.shared.approved.swap(false, Ordering::Relaxed)
    }

    /// 승인 표식 제거. 승인된 호출이 끝나면 표식이 다음 호출로 새지 않게 하는 자리
    pub fn clear_approved(&self) {
        self.shared.approved.store(false, Ordering::Relaxed);
    }

    /// 취소 토큰을 실행 판에 부착. 루프가 자기가 받은 토큰을 그대로 넘김
    pub fn attach_cancel(&self, cancel: &CancellationToken) {
        let mut slot = self
            .shared
            .cancel
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *slot = cancel.clone();
    }

    /// 취소 토큰 복사본을 반환. 승인 대기·cancel 자리가 같은 기준으로 듣게 하는 자리
    #[must_use]
    pub fn cancel_token(&self) -> CancellationToken {
        self.shared
            .cancel
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }
}
