//! T3 시험: 트레이트 서명 책임. 골격 트레이트를 구현해 흐름만 확인.
use std::future::Ready;
use std::time::Instant;

use snap_core::llm::ChatRequest;
use snap_core::llm::Delta;
use snap_core::llm::LlmClient;
use snap_core::llm::StreamOutcome;
use snap_core::llm::StreamReceiver;
use snap_core::llm::StreamTimings;
use snap_core::llm::TransportError;
use tokio_util::sync::CancellationToken;

use crate::request::request;

/// 시험용 스트림 수신자. 조각을 저장하고 끝난 뒤 합친 응답을 만든다.
struct RecordingReceiver {
    pieces: Vec<Vec<u8>>,
    is_closed: bool,
}

impl RecordingReceiver {
    /// 받은 조각이 하나도 없는 수신자.
    fn new() -> Self {
        Self {
            pieces: Vec::new(),
            is_closed: false,
        }
    }
}

impl StreamReceiver for RecordingReceiver {
    fn push(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
        if self.is_closed {
            return Err(TransportError::cancelled());
        }
        self.pieces.push(bytes.to_vec());
        Ok(())
    }

    fn finish(&mut self) -> Result<StreamOutcome, TransportError> {
        self.is_closed = true;
        let body = self.pieces.concat();
        let content = String::from_utf8_lossy(&body).into_owned();
        Ok(outcome_with_content(&content))
    }
}

/// 시험용 클라이언트. 요청을 받아 수신자를 연다.
struct RecordingClient;

impl LlmClient for RecordingClient {
    type Open<'a>
        = Ready<Result<RecordingReceiver, TransportError>>
    where
        Self: 'a;
    type Receiver = RecordingReceiver;

    fn open(&self, _request: &ChatRequest, _cancel: CancellationToken) -> Self::Open<'_> {
        std::future::ready(Ok(RecordingReceiver::new()))
    }
}

/// 내용만 채운 스트림 결과.
fn outcome_with_content(content: &str) -> StreamOutcome {
    let now = Instant::now();
    StreamOutcome {
        delta: Delta {
            content: content.to_owned(),
            ..Delta::default()
        },
        timings: StreamTimings {
            request_start: now,
            first_token: Some(now),
            last_token: Some(now),
        },
    }
}

#[tokio::test]
async fn open_returns_receiver_that_collects_pieces() {
    let cancel = CancellationToken::new();

    let open = RecordingClient.open(&request(), cancel);
    let Ok(mut receiver) = open.await else {
        panic!("test opens stream");
    };
    receiver.push(b"hello").expect("test accepts piece");
    let Ok(outcome) = receiver.finish() else {
        panic!("test finishes stream");
    };

    assert_eq!(outcome.delta.content, "hello", "receiver keeps pieces");
}

#[test]
fn closed_receiver_rejects_late_piece() {
    let mut receiver = RecordingReceiver::new();
    receiver.push(b"first").expect("test accepts piece");
    receiver.finish().expect("test finishes stream");

    let result = receiver.push(b"late");

    assert!(result.is_err(), "closed receiver rejects late piece");
}
