//! 백오프 대기 자리. 시간 대기와 취소 토큰을 `select!`(biased)로 경합시키는 한 책임만 담음.

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use crate::llm::TransportError;

/// 백오프 대기를 시간 대기와 취소 토큰의 `select!`(biased)로 잰다.
///
/// `biased;`로 취소를 앞에 두어 취소가 이미 켜져 있으면 대기를 채우지 않고 취소 오류로 끝냄.
pub(super) async fn wait_backoff(
    wait: Duration,
    cancel: &CancellationToken,
) -> Result<(), TransportError> {
    tokio::select! {
        biased;
        () = cancel.cancelled() => Err(TransportError::cancelled()),
        () = tokio::time::sleep(wait) => Ok(()),
    }
}
