//! 서버 종류별로 생각(reasoning) 옵션 필드를 요청 본문에 더하는 규칙만 둡니다(03-6 R2 감사 분리).
//!
//! 본문 조립 골격(`request_body.rs`)과 달리 엔드포인트 종류가 늘 때 바뀌므로 파일을 따로 둡니다
//! (dev-principles 2절 SRP).

use serde_json::{Map, Value};

use crate::config::{EndpointKind, Reasoning};

/// 공통 생각 값을 서버가 받는 필드 이름으로 넣습니다(D15).
pub(in crate::llm) fn insert_reasoning(
    map: &mut Map<String, Value>,
    kind: EndpointKind,
    reasoning: Reasoning,
) {
    let effort = reasoning_wire_value(reasoning);
    match kind {
        EndpointKind::OpenAi
        | EndpointKind::Ollama
        | EndpointKind::Vllm
        | EndpointKind::LlamaCpp => {
            map.insert("reasoning_effort".into(), Value::String(effort.to_owned()));
        }
        EndpointKind::OpenRouter => {
            let mut nested = Map::new();
            nested.insert("effort".into(), Value::String(effort.to_owned()));
            map.insert("reasoning".into(), Value::Object(nested));
        }
        EndpointKind::LmStudio => {
            // 공통 생각 요청 필드가 없어 넣지 않습니다. 필요하면 extra만 합칩니다.
        }
    }
}

/// 설정의 생각 옵션을 서버가 받는 문자열 값으로 바꿉니다.
pub(super) fn reasoning_wire_value(reasoning: Reasoning) -> &'static str {
    match reasoning {
        Reasoning::Off => "none",
        Reasoning::Low => "low",
        Reasoning::Medium => "medium",
        Reasoning::High => "high",
    }
}
