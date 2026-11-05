//! 선택 키가 고른 모델 값(02-7)을 요청에 채운다.
//!
//! 본문 골격 조립은 02-5의 `llm/request_body.rs`를 그대로 쓰고(재정의 금지), 이 파일은 값 채우기와
//! 스트림 열기 재사용만 한다. 전송·재시도 규칙이 그대로 이어져 취소·시간 초과의 동작이 02-5와 같다(AC 3).

use crate::llm::client::{CancellationToken, ChatRequest, ChatStream, LlmClient, TransportError};

use super::client_build::SelectedClient;

impl SelectedClient {
    /// 선택한 모델의 서버용 id와 샘플링·`reasoning`·`extra`를 채운 요청 사본을 만든다.
    ///
    /// `reasoning`과 `extra`는 예약 키이고 02-2 검증이 요청 쪽 값의 모델 이름 거절을 막으므로 충돌이 없다(D15).
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
    /// 모델 값을 채운 뒤 02-5 클라이언트로 스트림을 열어 전송·재시도·시간 규칙을 그대로 재사용한다.
    ///
    /// 안쪽 클라이언트가 선택 키 엔드포인트 주소만 갖므로 요청은 그 서버로만 간다(AC 1).
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
