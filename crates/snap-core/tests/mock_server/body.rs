//! 응답 스트림을 조각 단위로 내보내는 본문 자리. `http_body::Body` 구현만 담당.
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use axum::body::Bytes;
use http_body::Body;
use http_body::Frame;
use tokio::time::Sleep;
use tokio::time::sleep;

use super::script::StreamStep;
use super::script::StreamStop;
use super::stop_frame::frame_for_stop;
use super::stream_step::Step;

/// 응답 본문 스트림. 조각 사이 대기 시간을 지키며 흘려보냄.
pub(crate) struct ChunkBody {
    /// 흘려보낼 규칙.
    plan: Vec<StreamStep>,
    /// 다음에 볼 규칙 위치.
    next: usize,
    /// 규칙이 끝난 뒤 연결을 끊는 방식.
    stop: StreamStop,
    /// 진행 중인 조각 사이 대기. 없음이면 조각을 바로 보냄.
    gap: Option<Pin<Box<Sleep>>>,
}

impl ChunkBody {
    /// 규칙과 연결 종료 방식 본문.
    pub(crate) fn new(plan: Vec<StreamStep>, stop: StreamStop) -> Self {
        Self {
            plan,
            next: 0,
            stop,
            gap: None,
        }
    }

    /// 조각 사이 대기가 끝났는지 먼저 봄. 끝나면 다음 규칙을 처리함.
    fn poll_next_step(&mut self, cx: &mut Context<'_>) -> Poll<Step> {
        loop {
            let Some(gap) = self.gap.as_mut() else {
                return Poll::Ready(self.take_step());
            };
            if gap.as_mut().poll(cx).is_pending() {
                return Poll::Pending;
            }
            self.gap = None;
        }
    }

    /// 규칙 위치를 옮기며 처리 결과를 냄. 규칙이 없으면 종료 결과.
    fn take_step(&mut self) -> Step {
        let Some(step) = self.plan.get(self.next) else {
            self.next = self.plan.len();
            return Step::Finished(self.stop);
        };
        self.next += 1;
        match step {
            StreamStep::Chunk { bytes, gap } => {
                if !gap.is_zero() {
                    self.gap = Some(Box::pin(sleep(*gap)));
                }
                Step::Bytes(bytes.clone())
            }
            StreamStep::Stop(stop) => Step::Finished(*stop),
        }
    }
}

impl Body for ChunkBody {
    type Data = Bytes;

    type Error = std::io::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.get_mut().poll_next_step(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Step::Bytes(bytes)) => {
                Poll::Ready(Some(Ok(Frame::data(Bytes::from(bytes)))))
            }
            Poll::Ready(Step::Finished(stop)) => frame_for_stop(stop),
        }
    }
}
