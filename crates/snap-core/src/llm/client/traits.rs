//! 스트리밍 대화를 여는 클라이언트 트레이트입니다.

use crate::llm::client::CancellationToken;
use crate::llm::client::error::TransportError;
use crate::llm::client::request::ChatRequest;
use crate::llm::client::stream::ChatStream;

/// 스트리밍 대화를 여는 LLM 클라이언트입니다.
///
/// 상위는 구현체 종류를 몰라도 이 트레이트만으로 스트림을 엽니다.
pub trait LlmClient {
    /// 요청과 취소 토큰으로 이벤트 스트림을 엽니다.
    ///
    /// 헤더를 받기 전에 실패하면 `Err`입니다. 스트림을 연 뒤 실패는
    /// [`ChatStream::next_event`]의 `Err`로 나갑니다.
    fn stream_chat(
        &self,
        request: ChatRequest,
        cancel: CancellationToken,
    ) -> impl std::future::Future<Output = Result<ChatStream, TransportError>> + Send;
}
