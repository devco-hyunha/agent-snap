//! 스크립트대로 응답을 조립해 보내는 핸들러와 본문 스트림 구현입니다.
//!
//! 지연·멈춤·연결 종료를 테스트가 관찰할 수 있게, 조각 단위로 본문을 내보냅니다.
//! 스크립트 규격은 `super::script`, 연결 수 세기는 `super::server`가 맡습니다.

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{HeaderMap, Method, Uri};
use axum::response::Response;
use http_body::Frame;

use super::script::{MockBodyPart, RecordedRequest};
use super::server::{self, ConnectionGuard, Shared};

/// 스크립트대로 조각·지연을 내는 본문입니다.
struct ScriptBody {
    parts: std::vec::IntoIter<MockBodyPart>,
    sleep: Option<Pin<Box<tokio::time::Sleep>>>,
    pending_data: Option<Bytes>,
}

impl ScriptBody {
    fn new(parts: Vec<MockBodyPart>) -> Self {
        Self {
            parts: parts.into_iter(),
            sleep: None,
            pending_data: None,
        }
    }
}

impl http_body::Body for ScriptBody {
    type Data = Bytes;
    type Error = std::convert::Infallible;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = &mut *self;
        loop {
            if let Some(sleep) = this.sleep.as_mut() {
                match sleep.as_mut().poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(()) => {
                        this.sleep = None;
                        if let Some(data) = this.pending_data.take() {
                            return Poll::Ready(Some(Ok(Frame::data(data))));
                        }
                    }
                }
            }

            let Some(part) = this.parts.next() else {
                return Poll::Ready(None);
            };

            if part.delay_before.is_zero() {
                return Poll::Ready(Some(Ok(Frame::data(Bytes::from(part.bytes)))));
            }

            this.pending_data = Some(Bytes::from(part.bytes));
            this.sleep = Some(Box::pin(tokio::time::sleep(part.delay_before)));
        }
    }
}

/// 본문이 끝나거나 끊기면 연결 가드를 버리는 본문입니다.
struct GuardedBody {
    inner: ScriptBody,
    guard: Option<ConnectionGuard>,
}

impl http_body::Body for GuardedBody {
    type Data = Bytes;
    type Error = std::convert::Infallible;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
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

impl Drop for GuardedBody {
    fn drop(&mut self) {
        self.guard.take();
    }
}

pub(super) async fn handle_chat_completions(
    State(shared): State<Arc<Shared>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let guard = server::ConnectionGuard::new(&shared);

    {
        server::record(
            &shared,
            RecordedRequest {
                method: method.to_string(),
                path: uri.path().to_string(),
                headers: headers.clone(),
                body: body.to_vec(),
            },
        )
        .await;
    }

    let script = server::current_script(&shared).await;

    if script.hang_before_headers {
        // 클라이언트가 끊을 때까지 헤더를 보내지 않습니다.
        let _guard = guard;
        return std::future::pending::<Response>().await;
    }

    if !script.header_delay.is_zero() {
        tokio::time::sleep(script.header_delay).await;
    }

    let mut builder = Response::builder().status(script.status);
    for (name, value) in script.response_headers.iter() {
        builder = builder.header(name, value);
    }

    let guarded = GuardedBody {
        inner: ScriptBody::new(script.body_parts),
        guard: Some(guard),
    };
    builder
        .body(Body::new(guarded))
        .expect("모의 응답 본문 조립")
}
