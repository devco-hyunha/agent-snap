//! 취소 재료 자리. 취소 시각이 지나면 토큰을 끊는 창구만 담음.
use std::time::Duration;

use tokio_util::sync::CancellationToken;

/// 취소 시각이 지나면 끊기는 취소 토큰을 만들어 배경으로 띄움.
pub fn cancel_after(token: &CancellationToken, wait: Duration) {
    let signal = token.clone();
    tokio::spawn(async move {
        tokio::time::sleep(wait).await;
        signal.cancel();
    });
}
