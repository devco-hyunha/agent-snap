//! 클라이언트 트레이트 자리. 트레이트 골격과 스트림 수신 트레이트만 담음.

use std::future::Future;

use tokio_util::sync::CancellationToken;

use crate::llm::ChatRequest;
use crate::llm::TransportError;
use crate::llm::client::stream::StreamOutcome;

/// 스트림 조각을 받아 합치는 수신자. 전송 구현이 02-3 복원을 감싼 자리.
pub trait StreamReceiver {
    /// 바이트 조각 합침. 오류가 나면 스트림 종료.
    fn push(&mut self, bytes: &[u8]) -> Result<(), TransportError>;

    /// 스트림 끝. 마지막까지 합친 결과 반환.
    fn finish(&mut self) -> Result<StreamOutcome, TransportError>;
}

/// LLM 클라이언트 트레이트. 요청과 취소 토큰으로 스트림을 여는 계약.
pub trait LlmClient {
    /// 스트림을 여는 미래.
    type Open<'a>: Future<Output = Result<Self::Receiver, TransportError>>
    where
        Self: 'a;

    /// 스트림 조각을 합치는 수신자.
    type Receiver: StreamReceiver;

    /// 요청을 보내고 스트림 수신자 열기.
    fn open(&self, request: &ChatRequest, cancel: CancellationToken) -> Self::Open<'_>;
}
