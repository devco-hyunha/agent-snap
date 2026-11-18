//! 서버가 준 바이트 조각을 디코더에 넣어 프레임을 만드는 스트림 상태 기계.
//!
//! reqwest 타입은 이 파일 안에서만 씀. 순서: 대기 장 → 새 조각 → 디코더 마무리 → 정상 종료 장.

use std::collections::VecDeque;
use std::pin::Pin;
use std::time::{Duration, Instant};

use crate::llm::client::{
    CancellationToken, ChatDriver, ChatFrame, StreamOutcome, StreamTiming, TimeoutSegment,
    TransportError,
};
use crate::llm::codec::decode::StreamDecoder;
use crate::llm::codec::delta::{StreamEvent, Usage};

/// 서버가 준 바이트 조각을 디코더에 넣어 장을 만드는 전송 쪽 손잡이.
pub(crate) struct StreamState {
    /// 본문을 조각조각 읽는 응답. reqwest 타입은 이 파일 안에만 있음.
    pub response: reqwest::Response,
    /// 마무리(`finish`)를 한 번만 부르기 위해 `take`로 비우는 디코더.
    pub decoder: Option<StreamDecoder>,
    pub cancel: CancellationToken,
    pub idle: Duration,
    pub timing: StreamTiming,
    /// 만들어 놓고 아직 안 낸 장(앞에서부터 내보냄).
    pub pending: VecDeque<ChatFrame>,
    pub upstream_done: bool,
    /// 서버가 준 사용량. 없으면 `None`.
    pub usage: Option<Usage>,
    /// 마지막에 낼 정상 종료 장(끝난 뒤에도 사용량·시각은 남음).
    pub end_pending: bool,
    /// 첫 `Err` 이후에는 디코더를 안 쓰고 종료만 남긴다는 표식.
    pub failed: bool,
}

impl ChatDriver for StreamState {
    fn next_frame(&mut self) -> Pin<Box<dyn std::future::Future<Output = ChatFrame> + Send + '_>> {
        Box::pin(self.next())
    }
}

impl StreamState {
    /// 서버에서 다음 조각을 받아 다음 장 하나를 만듦.
    async fn next(&mut self) -> ChatFrame {
        loop {
            if let Some(frame) = self.pending.pop_front() {
                return frame;
            }
            if self.failed {
                return ChatFrame::End(None);
            }
            if self.end_pending {
                self.end_pending = false;
                return ChatFrame::End(Some(self.outcome()));
            }
            if self.upstream_done {
                self.finish_decoder();
                continue;
            }
            self.read_one_chunk().await;
        }
    }

    /// 새 조각을 하나 받아 디코더에 넣음. 장이 필요하면 pending에 채움.
    async fn read_one_chunk(&mut self) {
        let chunk = tokio::select! {
            result = self.response.chunk() => result,
            () = self.cancel.cancelled() => {
                self.mark_failure(TransportError::Cancelled);
                return;
            }
            () = tokio::time::sleep(self.idle) => {
                let error = TransportError::Timeout {
                    segment: TimeoutSegment::StreamIdle,
                    seconds: self.idle.as_secs(),
                };
                self.mark_failure(error);
                return;
            }
        };
        match chunk {
            Ok(Some(bytes)) => {
                let Some(decoder) = self.decoder.as_mut() else {
                    return;
                };
                match decoder.push(&bytes) {
                    Ok(events) => self.record_frames(events),
                    Err(error) => {
                        // 첫 Err 이후에는 디코더를 안 씀. 키·호스트 가리기는 이미 끝난 값.
                        self.mark_failure(TransportError::Streaming(error));
                    }
                }
            }
            Ok(None) => self.upstream_done = true,
            Err(_) => {
                // 읽는 중 연결 끊김. 주소가 들어갈 수 있는 오류 텍스트는 버림.
                self.mark_failure(TransportError::BodyRead);
            }
        }
    }

    /// 본문이 끝나 디코더만 남았을 때 마무리하고 남은 이벤트로 장을 만듦.
    fn finish_decoder(&mut self) {
        let Some(decoder) = self.decoder.take() else {
            self.end_pending = true;
            return;
        };
        match decoder.finish() {
            Ok((events, assembled)) => {
                self.usage = assembled.usage;
                self.record_frames(events);
                self.end_pending = true;
            }
            Err(error) => self.mark_failure(TransportError::Streaming(error)),
        }
    }

    /// 이벤트에 장을 만들어 붙이고, content·생각·도구 델타에는 도착 시각을 남김.
    fn record_frames(&mut self, events: Vec<StreamEvent>) {
        let now = Instant::now();
        for event in events {
            if is_token_event(&event) {
                if self.timing.first_token.is_none() {
                    self.timing.first_token = Some(now);
                }
                self.timing.last_token = Some(now);
            }
            self.pending.push_back(ChatFrame::Event(Ok(event)));
        }
    }

    /// 실패를 표시하고 아직 안 낸 장을 버린 뒤 오류 장 하나만 남김.
    fn mark_failure(&mut self, error: TransportError) {
        self.failed = true;
        self.pending.clear();
        self.pending.push_back(ChatFrame::Event(Err(error)));
    }

    /// 정상 종료 장에 담을 사용량·시각.
    fn outcome(&self) -> StreamOutcome {
        StreamOutcome {
            usage: self.usage.clone(),
            timing: self.timing.clone(),
        }
    }
}

/// content·생각·도구 델타를 토큰으로 봄. 사용량과 `[DONE]`은 세지 않음.
fn is_token_event(event: &StreamEvent) -> bool {
    matches!(
        event,
        StreamEvent::Content(_) | StreamEvent::Reasoning(_) | StreamEvent::ToolCallDelta { .. }
    )
}
