//! 읽기 태스크 결과를 wake 주기 안에 확인하는 자리
//!
//! wake 주기 안에 끝나지 않았으면 빈 조각으로 돌려보냄

use std::pin::Pin;
use std::time::Duration;

use tokio::task::JoinHandle;

/// 읽기 태스크 결과를 wake 주기 안에 확인. 끝나지 않았으면 빈 조각
///
/// 결과를 받은 태스크는 자리에서 빼 두고, wake마다 그 태스크를 한 번씩만 부름
#[must_use]
pub async fn pipe_text(slot: &mut Option<JoinHandle<String>>, wake_interval: Duration) -> String {
    let reader = match slot.as_mut() {
        Some(reader) => reader,
        None => return String::new(),
    };
    let reading = tokio::time::timeout(
        wake_interval,
        std::future::poll_fn(move |cx| Pin::new(&mut *reader).poll(cx)),
    );
    match reading.await {
        Ok(Ok(text)) => {
            drop(slot.take());
            text
        }
        Ok(Err(_)) | Err(_) => String::new(),
    }
}
