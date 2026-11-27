//! 서버 쪽 연결 정리 표시 재료. 연결을 받아 쓰는 스트림을 드롭 표시와 함께 감싼다.
//!
//! `axum::serve`가 쓰는 리스너를 감싸서, 연결이 끝날 때 표시를 남긴다.
//! 취소·시간 초과 시험이 서버 쪽 연결 닫힘을 확인하는 재료로만 쓴다.

use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::task::Context;
use std::task::Poll;

use axum::serve::Listener;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

/// 연결이 끝났는지 서버 쪽에서 확인하는 표시.
#[derive(Debug, Clone, Default)]
pub struct ClosedFlag(Arc<AtomicBool>);

impl ClosedFlag {
    /// 연결이 끝나지 않은 표시.
    pub fn new() -> Self {
        Self::default()
    }

    /// 연결이 끝났는지.
    pub fn is_closed(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

/// 연결 하나를 드롭 표시와 함께 감싼 스트림.
#[derive(Debug)]
pub struct CountedStream {
    /// 실제 소켓.
    stream: TcpStream,

    /// 이 연결이 끝나면 켜질 표시.
    closed: Arc<AtomicBool>,
}

impl Drop for CountedStream {
    /// 스트림이 드롭되면 서버 쪽 연결이 닫힌 표시를 켬.
    fn drop(&mut self) {
        self.closed.store(true, Ordering::SeqCst);
    }
}

impl AsyncRead for CountedStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for CountedStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        Pin::new(&mut this.stream).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.stream).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.stream).poll_shutdown(cx)
    }
}

/// 연결을 받아들일 때마다 드롭 표시를 붙이는 리스너.
#[derive(Debug)]
pub struct CountingListener {
    /// 실제 리스너.
    listener: TcpListener,

    /// 연결이 끝날 때마다 켜지는 표시.
    closed: Arc<AtomicBool>,
}

impl CountingListener {
    /// 끝남 표시를 함께 쓰는 리스너로 감쌈.
    pub fn new(listener: TcpListener, closed: &ClosedFlag) -> Self {
        Self {
            listener,
            closed: Arc::clone(&closed.0),
        }
    }
}

impl Listener for CountingListener {
    type Io = CountedStream;
    type Addr = SocketAddr;

    #[allow(clippy::expect_used)]
    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        let (stream, addr) = self
            .listener
            .accept()
            .await
            .expect("test listener accepts connection");
        (
            CountedStream {
                stream,
                closed: Arc::clone(&self.closed),
            },
            addr,
        )
    }

    fn local_addr(&self) -> std::io::Result<Self::Addr> {
        self.listener.local_addr()
    }
}
