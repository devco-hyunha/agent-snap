//! 청크 델타 필드 타입 자리. 청크 하나가 담는 필드 묶음·문자열 값 상태·도구 호출 델타만 담음.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::llm::delta::types::Usage;

/// 청크 하나가 담는 델타 필드 묶음.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkDelta {
    /// `delta.content` 원문 필드.
    pub content: FieldText,

    /// `delta.reasoning_content`와 `delta.reasoning` 원문 필드. (순서 유지용)
    pub reasoning: Vec<FieldText>,

    /// `delta.tool_calls` 델타 목록.
    pub tool_calls: Vec<ToolCallDelta>,

    /// `delta`에서 아는 필드를 뺀 나머지 원문 JSON.
    pub unknown_fields: BTreeMap<String, Value>,

    /// 청크 최상위 `usage`에서 읽은 사용량. 없으면 없음.
    pub usage: Option<Usage>,
}

/// 델타 필드의 문자열 값 상태.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldText {
    /// 문자열로 온 값.
    Text(String),

    /// 문자열이 아닌 값. (숫자·배열·객체·null)
    NotText,

    /// 필드 없음.
    Missing,
}

/// `delta.tool_calls`의 델타 하나.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallDelta {
    /// 델타가 가리키는 index.
    pub index: usize,

    /// `id` 원문 필드.
    pub id: FieldText,

    /// `function.name` 원문 필드.
    pub name: FieldText,

    /// `function.arguments` 원문 필드.
    pub arguments: FieldText,
}
