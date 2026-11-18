//! 서버 종류별로 생각(reasoning) 옵션 필드를 요청 본문에 더하는 규칙.

use serde_json::{Map, Value};

use crate::config::{EndpointKind, Reasoning};

/// 공통 생각 값을 서버가 받는 필드 이름으로 넣음.
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
            // 공통 생각 요청 필드가 없어 넣지 않음. 필요하면 extra만 합침.
        }
    }
}

/// 설정의 생각 옵션을 서버가 받는 문자열 값으로 바꿈.
pub(super) fn reasoning_wire_value(reasoning: Reasoning) -> &'static str {
    match reasoning {
        Reasoning::Off => "none",
        Reasoning::Low => "low",
        Reasoning::Medium => "medium",
        Reasoning::High => "high",
    }
}
