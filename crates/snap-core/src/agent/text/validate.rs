//! 2안 텍스트 프로토콜 응답의 스키마 검증 한 곳입니다(03-4 spec AC 4, D50 결정 4 ⑤·⑥).
//!
//! 등록 여부와 인자 스키마를 도구 실행 전에 걸러주는 판단만 맡습니다. 복원 래더는
//! `recover.rs`, 안내 글자는 `notice.rs`가 맡습니다(관심사 분리, dev-principles 5절).

use serde_json::Value;

use super::recover::RecoveredCall;
use crate::agent::registry::ToolRegistry;

/// 검증 걸러짐 표기입니다(안내 글자는 `notice`가 정본으로 만듭니다, spec AC 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallProblem {
    /// 등록에 없는 도구 이름입니다.
    Unregistered,
    /// 인자 스키마가 어긋났습니다. `reason`은 "필요한 것: …" 글자입니다.
    Argument {
        /// 어긋난 항목을 나열한 짧은 글자입니다.
        reason: String,
    },
}

/// 복원한 호출을 등록 목록과 스키마로 검증합니다(래더 ⑤·⑥, spec AC 4).
///
/// 등록에 없는 이름과 필수 누락·타입 어긋남을 도구 실행 전에 걸러줍니다. 통과하면 `None`,
/// 걸러지면 `CallProblem`으로 넘깁니다. 안내 글자는 `notice`의 정본이 만들므로,
/// 이 함수는 판단만 하고 글자는 만들지 않습니다(리뷰 3차 2번 지적, dev-principles 3절 DRY).
#[must_use]
pub fn validate_found(registry: &ToolRegistry, call: &RecoveredCall) -> Option<CallProblem> {
    if !registry.contains(&call.name) {
        return Some(CallProblem::Unregistered);
    }
    let schema = registry
        .find(&call.name)
        .map(|tool| tool.parameters_schema())
        .unwrap_or(Value::Null);
    argument_problem(&schema, &call.arguments).map(|reason| CallProblem::Argument { reason })
}

/// 스키마와 인자 JSON을 대조해 어긋난 항목을 "필요한 것: …" 한 줄로 돌려줍니다. 통과하면 None입니다.
///
/// 필수 누락과 타입 어긋남만 봅니다(AC 4). 인자 JSON이 객체가 아니거나 스키마에 `properties`가
/// 없으면 검사하지 않고 통과로 봅니다(인자 없는 도구는 검사할 항목이 없음).
#[must_use]
pub fn argument_problem(schema: &Value, arguments_json: &str) -> Option<String> {
    let Ok(Value::Object(object)) = serde_json::from_str::<Value>(arguments_json) else {
        return None;
    };
    let required: Vec<&str> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let missing: Vec<String> = required
        .iter()
        .filter(|key| object.get(&key.to_string()).is_none())
        .map(|key| (*key).to_owned())
        .collect();
    let mismatched = schema
        .get("properties")
        .and_then(Value::as_object)
        .map(|items| {
            object
                .iter()
                .filter(|(key, value)| !type_matches(items.get(key.as_str()), value))
                .map(|(key, _)| key.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let problems: Vec<String> = [missing, mismatched].concat();
    (!problems.is_empty()).then(|| format!("필요한 것: {}", problems.join(", ")))
}

/// 스키마 항목 정의와 JSON 값의 타입이 맞는지 봅니다. 정의가 없거나 알 수 없는 타입이면 검사하지 않습니다.
fn type_matches(property: Option<&Value>, value: &Value) -> bool {
    let declared = property
        .and_then(|item| item.get("type"))
        .and_then(Value::as_str)
        .unwrap_or("any");
    match declared {
        "string" => value.is_string(),
        "number" => value.is_number(),
        "integer" => value.as_i64().is_some(),
        "boolean" => value.is_boolean(),
        "array" => value.is_array(),
        "object" => value.is_object(),
        _ => true,
    }
}
