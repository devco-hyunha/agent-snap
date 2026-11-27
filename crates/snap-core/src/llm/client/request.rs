//! 채팅 요청 타입 자리. 필드만 담는 파일. 본문 조립은 02-5 T4가 맡고, 강도 변환 로직은 `effort.rs`가 맡음.

use serde_json::Map;
use serde_json::Value;

use crate::config::types::Sampling;

/// 채팅 요청 하나. 클라이언트 트레이트가 받는 입력 골격.
#[derive(Debug)]
pub struct ChatRequest {
    /// 서버에 보내는 모델 식별 값.
    pub model_id: String,

    /// 대화 메시지 순서대로.
    pub messages: Vec<ChatMessage>,

    /// 도구 정의 원문 JSON. 서버가 받는 그대로.
    pub tools: Vec<Value>,

    /// 샘플링 값. 없는 항목은 서버 기본.
    pub sampling: Sampling,

    /// 생각 강도. 없으면 관련 필드를 보내지 않음.
    pub reasoning: Option<ReasoningEffort>,

    /// 모델별 `extra` 원문. 본문 합치기는 T4.
    pub extra: Option<Map<String, Value>>,
}

/// 대화 메시지 하나. 역할과 본문.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    /// 메시지 역할. `system`·`user`·`assistant`·`tool`.
    pub role: String,

    /// 본문 텍스트.
    pub content: String,
}

/// 생각 강도 값. 설정이 허용하는 네 값만 다룸.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningEffort {
    /// 설정 값 `off`.
    Off,

    /// 설정 값 `low`.
    Low,

    /// 설정 값 `medium`.
    Medium,

    /// 설정 값 `high`.
    High,
}
