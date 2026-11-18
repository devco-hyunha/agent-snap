//! execute 브리지: 엔진 스레드의 결과를 지켜보는 워치독 창구.
//!
//! QuickJS 엔진을 건드리는 스레드는 워치독 하나여야 함(rlib2 병렬 제어 제약). 시간 상한은 `run_command`가 쓰는 값과 동일.

use std::sync::mpsc;
use std::time::Duration;

use super::policy::WATCHDOG_EXTRA_SECS;

/// 워치독 창구. 엔진 알림을 받고 시간 상한 초과를 판정.
pub(in crate::tools) fn watch_outcome(
    run_limit_secs: u64,
    receiver: &mpsc::Receiver<Result<String, String>>,
) -> Result<Option<String>, String> {
    let timeout = Duration::from_secs(run_limit_secs.saturating_add(WATCHDOG_EXTRA_SECS));
    match receiver.recv_timeout(timeout) {
        Ok(Ok(_)) => Ok(None),
        Ok(Err(reason)) => Ok(Some(reason)),
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(Some(format!(
            "미완료: execute JS 코드 실행 상한({}초) 초과",
            run_limit_secs
        ))),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err("execute 엔진이 코드를 예상 밖으로 멈췄습니다".to_owned())
        }
    }
}
