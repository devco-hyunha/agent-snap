//! 02-3 T4 스트림 오류 시험 묶음. 책임별 파일(스트림 오류·형식 오류·가리기·길이 상한)만 봄.

mod format;
mod limit;
mod redact;
mod stream;

use snap_core::llm::SseError;
use snap_core::llm::StreamState;

/// `error` 청크 payload로 스트림을 끝내고 오류를 돌려주는 흐름.
pub(super) fn stream_error_from(payload: &str) -> SseError {
    let mut stream = StreamState::new();
    let bytes = format!("data: {payload}\n\n").into_bytes();
    // 시험 실패로 밝히려는 자리라 unwrap 허용.
    #[allow(clippy::unwrap_used)]
    stream.push(&bytes).unwrap_err()
}
