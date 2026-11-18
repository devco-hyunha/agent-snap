//! 요청 본문에 실을 대화·모델 값 타입.

use std::collections::BTreeMap;

use crate::config::{ExtraValue, Reasoning, Sampling};

/// 한 번 스트리밍 대화 요청에 실을 값.
///
/// 주소·종류·키·시간 설정은 클라이언트 쪽. 여기에는 본문 대화·모델 값만 둠.
#[derive(Debug, Clone, PartialEq)]
pub struct ChatRequest {
    /// 서버에 보낼 모델 id.
    pub model: String,
    /// 대화 메시지 목록.
    pub messages: Vec<ChatMessage>,
    /// 도구 정의 목록. 없으면 빈 목록.
    pub tools: Vec<ToolDefinition>,
    /// 샘플링 값. `None` 항목은 본문에서 빼 서버 기본을 따름.
    pub sampling: Sampling,
    /// 생각 옵션. 없으면 관련 필드를 보내지 않음.
    pub reasoning: Option<Reasoning>,
    /// 본문에 그대로 합칠 추가 값.
    pub extra: BTreeMap<String, ExtraValue>,
}

/// 대화 메시지 한 줄.
///
/// `assistant`는 `tool_calls`, `tool`은 `tool_call_id`를 씀. 둘 다 없으면 내용만 보냄.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChatMessage {
    /// 역할 이름(`system`, `user`, `assistant`, `tool` 등).
    pub role: String,
    /// 본문 텍스트.
    pub content: String,
    /// `assistant`이 도구를 부른 자리. 없으면 빈 목록.
    pub tool_calls: Vec<AssistantToolCall>,
    /// `tool` 메시지가 답한 도구 호출 id. 없으면 보내지 않음.
    pub tool_call_id: Option<String>,
}

/// 모델이 부른 도구 호출 한 개(요청 본문 `assistant`용).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantToolCall {
    /// 서버가 준 호출 id. 결과 붙일 때 이 id를 그대로 씀.
    pub id: String,
    /// 호출한 도구 이름.
    pub name: String,
    /// 모델이 준 인자 JSON 원문.
    pub arguments: String,
}

/// 요청에 실을 도구 한 개의 정의.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolDefinition {
    /// 도구 이름.
    pub name: String,
    /// 도구 설명.
    pub description: String,
    /// 인자 JSON Schema 객체.
    pub parameters: serde_json::Value,
}
