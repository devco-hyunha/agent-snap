//! 정상 응답으로 스트림을 여는 책임입니다.
//!
//! 요청 시각·첫 장 표기·디코더를 스트림 상태에 연결해 스트림을 만듭니다(02-6).

use std::collections::VecDeque;
use std::time::Instant;

use crate::llm::client::{CancellationToken, ChatStream, StreamTiming};
use crate::llm::codec::decode::StreamDecoder;
use crate::llm::transport::stream_state::StreamState;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 정상 응답으로 스트림을 만듭니다.
    pub(super) fn open_stream(
        &self,
        response: reqwest::Response,
        request_started: Instant,
        cancel: CancellationToken,
    ) -> ChatStream {
        ChatStream::new(Box::new(StreamState {
            response,
            decoder: Some(StreamDecoder::new()),
            cancel,
            idle: self.http.stream_idle_seconds,
            timing: StreamTiming {
                request_started,
                first_token: None,
                last_token: None,
            },
            pending: VecDeque::new(),
            upstream_done: false,
            usage: None,
            end_pending: false,
            failed: false,
        }))
    }
}
