//! `kind`별 `reasoning` 변환 규칙 자리. 변환 한 책임만 담음.

use serde_json::Map;
use serde_json::Value;

use crate::consts::request::REASONING_EFFORT_INNER_KEY;
use crate::consts::request::REASONING_EFFORT_KEY;
use crate::consts::request::REASONING_OBJECT_KEY;
use crate::consts::request::REASONING_VALUE_HIGH;
use crate::consts::request::REASONING_VALUE_LOW;
use crate::consts::request::REASONING_VALUE_MEDIUM;
use crate::consts::request::REASONING_VALUE_OFF;
use crate::llm::client::kind::EndpointKind;
use crate::llm::client::request::ReasoningEffort;

/// 생각 강도를 엔드포인트 형식에 맞는 본문 필드로 바꿔 본문에 합침.
///
/// 강도가 없으면 관련 필드를 넣지 않는다. `lmstudio`는 공통 thinking 필드가 없다.
pub fn attach_reasoning(
    body: &mut Map<String, Value>,
    kind: EndpointKind,
    effort: Option<ReasoningEffort>,
) {
    let Some(effort) = effort else {
        return;
    };
    let value = Value::String(value_text(effort).to_owned());
    match kind {
        EndpointKind::Openai
        | EndpointKind::Ollama
        | EndpointKind::Vllm
        | EndpointKind::Llamacpp => {
            body.insert(REASONING_EFFORT_KEY.to_owned(), value);
        }
        EndpointKind::Openrouter => {
            let mut object = Map::new();
            object.insert(REASONING_EFFORT_INNER_KEY.to_owned(), value);
            body.insert(REASONING_OBJECT_KEY.to_owned(), Value::Object(object));
        }
        EndpointKind::Lmstudio => {}
    }
}

/// 본문에 담는 강도 값 문자열. `off`는 `none`으로 바꿈.
const fn value_text(effort: ReasoningEffort) -> &'static str {
    match effort {
        ReasoningEffort::Off => REASONING_VALUE_OFF,
        ReasoningEffort::Low => REASONING_VALUE_LOW,
        ReasoningEffort::Medium => REASONING_VALUE_MEDIUM,
        ReasoningEffort::High => REASONING_VALUE_HIGH,
    }
}
