//! `ToolContext`의 승인 표식·취소 토큰 관리 자리입니다(03-6).
//!
//! `tool.rs`는 트레이트 골격만 두고, 승인 상태와 취소 토큰을 다루는 메서드는 이 파일에
//! 따로 둡니다(dev-principles 2절 SRP, 03-6 R1 감사 분리). 메서드 이름과 서명은 그대로이고
//! 공개 경로(`ToolContext`의 메서드)도 바뀌지 않습니다.

use std::sync::atomic::Ordering;

use tokio_util::sync::CancellationToken;

use crate::tools::tool::ToolContext;

impl ToolContext {
    /// 03-6 승인 흐름이 "이 호출은 승인받았다" 표식을 채우는 자리입니다.
    pub fn mark_approved(&self) {
        self.approved.store(true, Ordering::Relaxed);
    }

    /// 승인 표식을 확인해 승인된 호출이면 소비합니다. `run_command`이 자기 흐름 판단에 씁니다.
    ///
    /// 확인하는 자마다 표식을 소비하므로, 승인자 응답이 승인한 뒤의 재호출 한 개에만 표식이 쓰이고
    /// 뒤따르는 호출은 다시 승인을 받아야 합니다(D22 5.3절·spec AC 1: 명령마다 확인).
    pub fn take_approved(&self) -> bool {
        self.approved.swap(false, Ordering::Relaxed)
    }

    /// 승인 표식을 지웁니다. 승인된 호출이 끝나면 표식이 다음 호출으로 새지 않게 하는 자리입니다.
    pub fn clear_approved(&self) {
        self.approved.store(false, Ordering::Relaxed);
    }

    /// 02 취소 토큰을 실행 판에 붙입니다. 루프(03-2)가 자기가 받은 토큰을 그대로 넘깁니다(spec AC 4).
    pub fn attach_cancel(&self, cancel: &CancellationToken) {
        let mut slot = self
            .cancel
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *slot = cancel.clone();
    }

    /// 02 취소 토큰 복사본을 돌려줍니다. 03-6 승인 대기·cancel 자리가 같은 기준으로 듣게 하는 자리입니다.
    #[must_use]
    pub fn cancel_token(&self) -> CancellationToken {
        self.cancel
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }
}
