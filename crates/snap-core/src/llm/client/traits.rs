//! 스트리밍 대화를 여는 클라이언트 트레이트.

use crate::llm::client::CancellationToken;
use crate::llm::client::error::TransportError;
use crate::llm::client::request::ChatRequest;
use crate::llm::client::stream::ChatStream;

/// 스트리밍 대화를 여는 LLM 클라이언트.
///
/// 상위는 구현체 종류 없이 이 트레이트만으로 스트림을 염.
pub trait LlmClient {
    /// 요청과 취소 토큰으로 이벤트 스트림을 염.
    ///
    /// 헤더 전 실패는 `Err`. 연 뒤 실패는 [`ChatStream::next_event`]의 `Err`.
    fn stream_chat(
        &self,
        request: ChatRequest,
        cancel: CancellationToken,
    ) -> impl std::future::Future<Output = Result<ChatStream, TransportError>> + Send;
}
