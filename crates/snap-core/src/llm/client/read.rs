//! 스트림 조각 읽기 자리. 유휴 시간·취소·읽기를 한 번씩 경합시키는 한 책임만 담음.
//!
//! reqwest `read_timeout`은 구간을 가리지 못해 미사용. 유휴는 이 파일이 tokio로 측정.

use reqwest::Response;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::llm::TransportError;
use crate::llm::client::connect::connect_error;
use crate::llm::client::timings::TransportTimings;
use crate::llm::client::traits::StreamReceiver;

/// 응답 본문을 끝까지 읽어 수신자에 넣고 결과를 냄.
///
/// 조각이 없는 시간(유휴)은 `stream_idle`로 재고, 취소는 같은 자리에서 경합.
/// 취소·유휴로 끝나면 응답 값을 버려 연결을 정리한다.
pub async fn read_stream<R: StreamReceiver>(
    response: &mut Response,
    receiver: &mut R,
    timings: &TransportTimings,
    cancel: &CancellationToken,
) -> Result<(), TransportError> {
    loop {
        let piece = tokio::select! {
            () = cancel.cancelled() => return Err(TransportError::cancelled()),
            read = timeout(timings.stream_idle, response.chunk()) => {
                match read {
                    Ok(Ok(Some(bytes))) => Some(bytes),
                    Ok(Ok(None)) => return Ok(()),
                    Ok(Err(error)) => return Err(connect_error(&error, timings.connect_seconds())),
                    Err(_) => return Err(TransportError::idle(timings.stream_idle_seconds())),
                }
            }
        };
        let Some(bytes) = piece else {
            return Ok(());
        };
        receiver.push(&bytes)?;
    }
}
