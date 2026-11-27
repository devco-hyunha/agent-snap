//! 종료 규칙을 본문 프레임으로 바꾸는 자리. 변환 함수 하나만 담당.
use std::task::Poll;

use axum::body::Bytes;
use http_body::Frame;

use super::script::StreamStop;

/// 규칙 종료 방식을 프레임으로 바꿈. 모든 규칙은 본문 끝 표시로 나간다.
/// 끊기 방식의 연결 정리는 서버 쪽이 담당한다.
/// 급히 끊는 방식은 본문 끝 표시를 보내지 않고 본문 읽기를 실패로 끝낸다.
pub(crate) fn frame_for_stop(
    stop: StreamStop,
) -> Poll<Option<Result<Frame<Bytes>, std::io::Error>>> {
    match stop {
        StreamStop::End | StreamStop::Cut | StreamStop::Drop => Poll::Ready(None),
        StreamStop::Abort => Poll::Ready(Some(Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "mock aborts response body",
        )))),
    }
}
