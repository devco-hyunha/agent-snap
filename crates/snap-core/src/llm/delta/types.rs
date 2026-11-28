//! 델타 복원 결과 타입 자리. 합쳐진 하나의 응답·index별 도구 호출·사용량을 담음.

use std::collections::BTreeMap;

use serde_json::Value;

/// 청크 델타를 끝까지 합친 하나의 응답.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Delta {
    /// content를 보낸 순서대로 이어 붙인 문자열.
    pub content: String,

    /// 생각을 보낸 순서대로 이어 붙인 문자열.
    pub reasoning: String,

    /// index별로 합친 도구 호출. 받은 순서대로 둠.
    pub tool_calls: Vec<ToolCall>,

    /// 모르는 델타 필드의 원문 JSON. 키별로 가장 최근 값을 둠.
    pub unknown_fields: BTreeMap<String, Value>,

    /// 청크 최상위 `usage`에서 읽은 사용량. 한 번도 없으면 없음.
    pub usage: Option<Usage>,
}

/// 청크 최상위 `usage`가 담는 사용량 숫자 셋.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Usage {
    /// `prompt_tokens` 숫자. 숫자가 아니면 없음.
    pub prompt_tokens: Option<u64>,

    /// `completion_tokens` 숫자. 숫자가 아니면 없음.
    pub completion_tokens: Option<u64>,

    /// `total_tokens` 숫자. 숫자가 아니면 없음.
    pub total_tokens: Option<u64>,
}

/// index별로 합친 도구 호출 하나.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCall {
    /// 델타에서 받은 index.
    pub index: usize,

    /// 합친 id. 첫 번째로 온 값만 남김.
    pub id: Option<String>,

    /// 합친 함수 이름.
    pub name: String,

    /// 합친 인자 문자열.
    pub arguments: String,
}
