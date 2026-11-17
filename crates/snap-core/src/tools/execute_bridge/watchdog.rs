//! 03-8 execute 브리지: 엔진 스레드의 결과를 지켜보는 워치독 창구만 둡니다(AC 6).
//!
//! 결론: QuickJS 엔진을 건드리는 스레드는 워치독 하나여야 합니다(rlib2 병렬 락, AC 6).
//! 엔진이 결과를 보내면 결과를 받고, 시간 상한이 지나면 엔진 스레드가 마칠 때까지
//! 기다리는 중단 사유를 돌려줍니다. 시간 상한 값은 03-6 `run_command`이 쓰는 값을 함께 씁니다.

use std::sync::mpsc;
use std::time::Duration;

use super::policy::WATCHDOG_EXTRA_SECS;

/// 워치독 창구입니다. 시간 상한·취소를 지켜보고 필요하면 중단 사유를 돌려줍니다.
///
/// 엔진이 결과를 보내면 결과를 받고, 시간 상한이 지나면 엔진 스레드가 마칠 때까지
/// 기다리는 중단 사유를 돌려줍니다.
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
