//! 선택 키가 고른 모델 값을 요청에 채움.
//!
//! 본문 골격은 `llm/request_body.rs`를 쓰고, 이 파일은 값 채우기와 스트림 열기 재사용만 함.

use crate::llm::client::{CancellationToken, ChatRequest, ChatStream, LlmClient, TransportError};

use super::client_build::SelectedClient;

impl SelectedClient {
    /// 선택한 모델의 서버용 id와 샘플링·`reasoning`·`extra`를 채운 요청 사본을 만듦.
    fn apply_selected(&self, request: &ChatRequest) -> ChatRequest {
        ChatRequest {
            model: self.model_id.clone(),
            messages: request.messages.clone(),
            tools: request.tools.clone(),
            sampling: self.sampling,
            reasoning: self.reasoning,
            extra: self.extra.clone(),
        }
    }
}

impl LlmClient for SelectedClient {
    /// 모델 값을 채운 뒤 안쪽 클라이언트로 스트림을 열어 전송·재시도·시간 규칙을 재사용.
    ///
    /// 안쪽 클라이언트가 선택 키 엔드포인트 주소만 갖므로 요청은 그 서버로만 감.
    async fn stream_chat(
        &self,
        request: ChatRequest,
        cancel: CancellationToken,
    ) -> Result<ChatStream, TransportError> {
        self.inner
            .stream_chat(self.apply_selected(&request), cancel)
            .await
    }
}
