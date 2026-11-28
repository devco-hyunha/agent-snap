//! 시험용 `ChatRequest` 골격 정본. 요청을 만드는 한 책임만 담음.
//! 같은 시험 묶음의 본문·변환·병합·전송·트레이트 시험 공용.
//! 전송 시험이 쓰는 시험용 키 값도 이 골격 재료로 둔다.

use snap_core::config::Sampling;
use snap_core::llm::ChatMessage;
use snap_core::llm::ChatRequest;
use snap_core::llm::ReasoningEffort;

/// 시험용 모델 식별 값.
pub(crate) const MODEL_ID: &str = "model-id";

/// 시험용 키 값. 모의 서버가 받은 헤더로만 확인.
pub(crate) const KEY: &str = "sk-testvalue123";

/// 강도·extra 없는 요청 골격.
pub(crate) fn request() -> ChatRequest {
    request_with(None, None)
}

/// 강도와 extra를 지정한 요청 골격. 골격 필드의 정본.
pub(crate) fn request_with(
    reasoning: Option<ReasoningEffort>,
    extra: Option<serde_json::Map<String, serde_json::Value>>,
) -> ChatRequest {
    ChatRequest {
        model_id: MODEL_ID.to_owned(),
        messages: Vec::new(),
        tools: Vec::new(),
        sampling: sampling(),
        reasoning,
        extra,
    }
}

/// 강도만 채운 요청 골격.
pub(crate) fn request_with_reasoning(reasoning: Option<ReasoningEffort>) -> ChatRequest {
    request_with(reasoning, None)
}

/// extra만 채운 요청 골격.
pub(crate) fn request_with_extra(extra: serde_json::Map<String, serde_json::Value>) -> ChatRequest {
    request_with(None, Some(extra))
}

/// 사용자 메시지 하나만 채운 요청 골격.
pub(crate) fn request_with_user_message() -> ChatRequest {
    let mut request = request();
    request.messages = vec![user_message()];
    request
}

/// 사용자 메시지 하나.
pub(crate) fn user_message() -> ChatMessage {
    ChatMessage {
        role: "user".to_owned(),
        content: "hello".to_owned(),
    }
}

/// 값이 있는 샘플링 하나. 전송 시험이 서버에 받는 그대로 확인용.
pub(crate) fn sampling_with_values() -> Sampling {
    Sampling {
        temperature: Some(0.5),
        top_p: None,
        max_tokens: Some(64),
    }
}

/// 샘플링 값이 비어 있는 골격.
fn sampling() -> Sampling {
    Sampling {
        temperature: None,
        top_p: None,
        max_tokens: None,
    }
}
