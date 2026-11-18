#![allow(dead_code, unreachable_pub)]

//! 스크립트 본문 조각을 순서·지연대로 내보내고 끊김을 흉내 내는 본문 스트림
//!
//! 규격은 `super::script`, 끊김 표식은 `super::disconnect`, 연결 수 세기는 `super::server`

use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec::IntoIter;

use axum::body::Bytes;
use http_body::Frame;

use super::disconnect::BodyDisconnected;
use super::guard::ConnectionGuard;
use super::script::MockBodyPart;

/// 본문 폴 결과를 한 형태로만 부름. 본문 종류마다 같은 폴 결과를 다시 적지 않음
type BodyPoll = Poll<Option<Result<Frame<Bytes>, BodyDisconnected>>>;

/// 조각 바이트를 본문 프레임으로 감쌈.
fn frame(data: Bytes) -> Frame<Bytes> {
    Frame::data(data)
}

/// 스크립트 조각을 순서대로 내보내는 공통 상태
///
/// 지연 예약·전송을 한곳에만 둠. 본문 종류가 같은 지연 코드를 복제하지 않음
struct ScriptParts {
    parts: IntoIter<MockBodyPart>,
    sleep: Option<Pin<Box<tokio::time::Sleep>>>,
    pending_data: Option<Bytes>,
}

impl ScriptParts {
    fn new(parts: Vec<MockBodyPart>) -> Self {
        Self {
            parts: parts.into_iter(),
            sleep: None,
            pending_data: None,
        }
    }

    /// 예약해 둔 조각을 보낼 준비가 됐는지 봄.
    ///
    /// 대기가 끝나면 예약한 조각을 돌려줌.
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Option<Bytes>> {
        if let Some(sleep) = self.sleep.as_mut() {
            if sleep.as_mut().poll(cx).is_pending() {
                return Poll::Pending;
            }
            self.sleep = None;
            return Poll::Ready(self.pending_data.take());
        }
        Poll::Ready(None)
    }

    /// 조각 하나를 보냄. 지연이 있으면 대기를 예약함
    fn take_part(&mut self, part: MockBodyPart) -> Option<Bytes> {
        if part.delay_before.is_zero() {
            return Some(Bytes::from(part.bytes));
        }
        self.pending_data = Some(Bytes::from(part.bytes));
        self.sleep = Some(Box::pin(tokio::time::sleep(part.delay_before)));
        None
    }

    /// 끊기 직전에 예약만 해 둔 조각을 꺼냄. 대기는 버림
    fn take_pending(&mut self) -> Option<Bytes> {
        self.sleep = None;
        self.pending_data.take()
    }
}

/// 스크립트대로 조각·지연을 내는 본문
///
/// 끊김 표식이 섞인 스크립트는 핸들러가 끊김 본문(`DisconnectBody`)으로 나눠 보내므로 여기엔 도착하지 않음
pub(super) struct ScriptBody {
    stream: ScriptParts,
}

impl ScriptBody {
    pub(super) fn new(parts: Vec<MockBodyPart>) -> Self {
        Self {
            stream: ScriptParts::new(parts),
        }
    }

    /// 예약된 조각을 먼저 보고, 그다음 조각을 하나 받아내는 공용 순서
    ///
    /// 본문 폴의 뼈대를 본문 종류마다 복제하지 않으려고 한곳에 둠. 끝내기 신호는 `Err(())`
    fn poll_next_part(stream: &mut ScriptParts, cx: &mut Context<'_>) -> Poll<Result<Bytes, ()>> {
        let ready = stream.poll_ready(cx);
        match ready {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Some(data)) => return Poll::Ready(Ok(data)),
            Poll::Ready(None) => {}
        }
        match stream.parts.next() {
            Some(part) if part.disconnect => Poll::Ready(Err(())),
            Some(part) => match stream.take_part(part) {
                Some(data) => Poll::Ready(Ok(data)),
                None => Poll::Pending,
            },
            None => Poll::Ready(Err(())),
        }
    }
}

impl http_body::Body for ScriptBody {
    type Data = Bytes;
    /// 본문 오류 타입은 연결 끊기 표식
    /// hyper가 본문 오류에서 연결을 닫아 받는 쪽에 끊김으로 보임
    type Error = BodyDisconnected;

    fn poll_frame(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> BodyPoll {
        let this = &mut *self;
        match Self::poll_next_part(&mut this.stream, cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(data)) => Poll::Ready(Some(Ok(frame(data)))),
            Poll::Ready(Err(())) => Poll::Ready(None),
        }
    }
}

/// 본문이 끝나거나 끊기면 연결 가드를 버리는 본문
///
/// 앞 조각을 다 보낸 뒤 끊는 끊김 본문과 연결 세기를 한자리에서 같이함
pub(super) struct GuardedBody<B> {
    pub(super) inner: B,
    pub(super) guard: Option<ConnectionGuard>,
}

impl<B> http_body::Body for GuardedBody<B>
where
    B: http_body::Body<Data = Bytes, Error = BodyDisconnected> + Unpin,
{
    type Data = Bytes;
    type Error = BodyDisconnected;

    fn poll_frame(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> BodyPoll {
        let this = &mut *self;
        match Pin::new(&mut this.inner).poll_frame(cx) {
            Poll::Ready(None) => {
                this.guard.take();
                Poll::Ready(None)
            }
            other => other,
        }
    }
}

impl<B> Drop for GuardedBody<B> {
    fn drop(&mut self) {
        self.guard.take();
    }
}

/// 앞 조각을 다 보낸 뒤 본문 마무리 없이 연결을 끊는 본문
///
/// hyper는 body 끝 마커를 못 본 클라이언트가 끊김을 알게 함. 오류 직전 폴에서 한 번 대기해 앞 조각을 소켓에 다 씀
pub(super) struct DisconnectBody {
    stream: ScriptParts,
    /// 끊기 직전 대기 표식
    /// hyper가 쓴 것을 소켓에 다 쓰게 한 자리에서 한 번만 씀
    flush_wakeup: bool,
}

impl DisconnectBody {
    pub(super) fn new(parts: Vec<MockBodyPart>) -> Self {
        Self {
            stream: ScriptParts::new(parts),
            flush_wakeup: false,
        }
    }

    /// 끊기 전에 보낸 조각을 클라이언트가 먼저 받게 함
    ///
    /// 이 폴에서 대기를 돌려 hyper가 쓴 것을 소켓에 다 쓰게 하고, 다음 폴에서 끊음
    fn flush_then_disconnect(&mut self, cx: &mut Context<'_>) -> BodyPoll {
        if self.flush_wakeup {
            return Poll::Ready(Some(Err(BodyDisconnected)));
        }
        self.flush_wakeup = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

impl http_body::Body for DisconnectBody {
    type Data = Bytes;
    type Error = BodyDisconnected;

    fn poll_frame(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> BodyPoll {
        let this = &mut *self;
        match ScriptBody::poll_next_part(&mut this.stream, cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(data)) => Poll::Ready(Some(Ok(frame(data)))),
            Poll::Ready(Err(())) => {
                // 보내지 않은 앞 조각은 버리지 않고 먼저 보냄
                if let Some(data) = this.stream.take_pending() {
                    return Poll::Ready(Some(Ok(frame(data))));
                }
                // 끊김 표식이 없는 스크립트는 끝까지 보내고 나서 끊음
                this.flush_then_disconnect(cx)
            }
        }
    }
}
