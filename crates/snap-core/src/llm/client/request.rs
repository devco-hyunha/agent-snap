//! 요청 본문에 실을 대화·모델 값 타입입니다.

use std::collections::BTreeMap;

use crate::config::{ExtraValue, Reasoning, Sampling};

/// 한 번 스트리밍 대화 요청에 실을 값입니다.
///
/// 엔드포인트 주소·종류·키·시간 설정은 클라이언트 쪽에 두고,
/// 여기에는 요청 본문에 들어갈 대화·모델 값만 둡니다.
#[derive(Debug, Clone, PartialEq)]
pub struct ChatRequest {
    /// 서버에 보낼 모델 id입니다.
    pub model: String,
    /// 대화 메시지 목록입니다.
    pub messages: Vec<ChatMessage>,
    /// 도구 정의 목록입니다. 없으면 빈 목록입니다.
    pub tools: Vec<ToolDefinition>,
    /// 샘플링 값입니다. `None` 항목은 본문에서 빼 서버 기본을 따릅니다.
    pub sampling: Sampling,
    /// 생각 옵션입니다. 없으면 관련 필드를 보내지 않습니다.
    pub reasoning: Option<Reasoning>,
    /// 본문에 그대로 합칠 추가 값입니다.
    pub extra: BTreeMap<String, ExtraValue>,
}

/// 대화 메시지 한 줄입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    /// 역할 이름입니다(`system`, `user`, `assistant`, `tool` 등).
    pub role: String,
    /// 본문 텍스트입니다.
    pub content: String,
}

/// 요청에 실을 도구 한 개의 정의입니다.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolDefinition {
    /// 도구 이름입니다.
    pub name: String,
    /// 도구 설명입니다.
    pub description: String,
    /// 인자 JSON Schema 객체입니다.
    pub parameters: serde_json::Value,
}
