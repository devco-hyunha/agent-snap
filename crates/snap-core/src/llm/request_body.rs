//! `/chat/completions` 요청 본문 조립입니다.
//!
//! 서버 종류(`kind`)에 맞춰 생각 옵션을 바꾸고, `extra`를 합칩니다(D15).
//! `tool_choice`·`n`·`stream_options`·맥락 한도는 넣지 않습니다.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::config::extra_json::{extra_to_json, json_f64};
use crate::config::{EndpointKind, ExtraValue, Reasoning, Sampling};
use crate::llm::client::{ChatMessage, ChatRequest, ToolDefinition};

/// 스트리밍 대화 요청 본문을 JSON 값으로 만듭니다.
pub(crate) fn build_chat_completions_body(kind: EndpointKind, request: &ChatRequest) -> Value {
    let mut map = Map::new();
    map.insert("model".into(), Value::String(request.model.clone()));
    map.insert("messages".into(), messages_json(&request.messages));
    map.insert("stream".into(), Value::Bool(true));
    if !request.tools.is_empty() {
        map.insert("tools".into(), tools_json(&request.tools));
    }
    insert_sampling(&mut map, &request.sampling);
    if let Some(reasoning) = request.reasoning {
        insert_reasoning(&mut map, kind, reasoning);
    }
    merge_extra(&mut map, &request.extra);
    Value::Object(map)
}

fn messages_json(messages: &[ChatMessage]) -> Value {
    Value::Array(
        messages
            .iter()
            .map(|message| {
                let mut item = Map::new();
                item.insert("role".into(), Value::String(message.role.clone()));
                item.insert("content".into(), Value::String(message.content.clone()));
                Value::Object(item)
            })
            .collect(),
    )
}

fn tools_json(tools: &[ToolDefinition]) -> Value {
    Value::Array(
        tools
            .iter()
            .map(|tool| {
                let mut function = Map::new();
                function.insert("name".into(), Value::String(tool.name.clone()));
                function.insert(
                    "description".into(),
                    Value::String(tool.description.clone()),
                );
                function.insert("parameters".into(), tool.parameters.clone());

                let mut item = Map::new();
                item.insert("type".into(), Value::String("function".into()));
                item.insert("function".into(), Value::Object(function));
                Value::Object(item)
            })
            .collect(),
    )
}

fn insert_sampling(map: &mut Map<String, Value>, sampling: &Sampling) {
    if let Some(temperature) = sampling.temperature {
        map.insert("temperature".into(), json_f64(temperature));
    }
    if let Some(top_p) = sampling.top_p {
        map.insert("top_p".into(), json_f64(top_p));
    }
    if let Some(max_tokens) = sampling.max_tokens {
        map.insert("max_tokens".into(), Value::from(max_tokens));
    }
}

/// 공통 생각 값을 서버가 받는 필드 이름으로 넣습니다(D15).
fn insert_reasoning(map: &mut Map<String, Value>, kind: EndpointKind, reasoning: Reasoning) {
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

fn reasoning_wire_value(reasoning: Reasoning) -> &'static str {
    match reasoning {
        Reasoning::Off => "none",
        Reasoning::Low => "low",
        Reasoning::Medium => "medium",
        Reasoning::High => "high",
    }
}

fn merge_extra(map: &mut Map<String, Value>, extra: &BTreeMap<String, ExtraValue>) {
    for (key, value) in extra {
        map.insert(key.clone(), extra_to_json(value));
    }
}
