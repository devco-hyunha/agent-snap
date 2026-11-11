//! 2안 텍스트 프로토콜 시스템 프롬프트 조립 한 곳입니다(03-4 spec AC 2, D50 결정 8).
//!
//! OpenAI `tools` 배열을 쓰지 못하는 로컬 모델에게 도구 이름·인자 형식·호출 예시를 글자로
//! 알려줍니다. 시스템 프롬프트 끝에 "도구 호출 형식" 블록을 붙이고, 지시문에 "JSON" 글자를
//! 명시합니다(12번 문서 8절 업계 관례).
//!
//! 프롬프트 앞머리·사용자 줄을 만드는 일은 `prompt.rs`의 책임이고, 이 파일은 2안 지시 블록만
//! 만듭니다(관심사 분리, dev-principles 5절).

use serde_json::Value;

use crate::llm::ChatRequest;

use crate::agent::registry::ToolRegistry;

/// 1안 `tools` 배열을 빼고 시스템 프롬프트 끝에 2안 지시 블록을 붙입니다(전환 조립, D50 결정 1).
pub fn apply_text_assembly(request: &mut ChatRequest, registry: &ToolRegistry) {
    request.tools.clear();
    let block = text_protocol_block(registry);
    if let Some(system) = request
        .messages
        .iter_mut()
        .find(|message| message.role == "system")
    {
        system.content.push('\n');
        system.content.push_str(&block);
    }
}

/// 2안 지시 블록 앞머리입니다. 모델이 답에 JSON 객체 하나만 쓰라고 지시합니다(spec AC 2).
pub const TEXT_PROTOCOL_HEAD: &str = "도구 호출 형식";

/// 2안 지시 블록 끝부분입니다. "JSON" 글자를 명시해 모델이 쓸 답을 정해줍니다(D50 결정 8).
pub const TEXT_PROTOCOL_RULES: &str =
    "답에는 JSON 객체 하나만 써줘. 여러 도구를 한 번에 부르지 마.";

/// 한 번에 노출하는 도구로 권하는 상한입니다(OpenAI 공식 안내, 12번 문서 8절, D50 결정 8).
pub const TEXT_PROTOCOL_TOOL_GUIDE_LIMIT: usize = 20;

/// 2안 시스템 프롬프트 끝에 "도구 호출 형식" 블록을 붙입니다(spec AC 2).
///
/// 등록한 도구 전부를 넣습니다. 한 도구당 3줄(이름과 설명, 인자 필드와 타입, 호출 예시 JSON)로
/// 쓰고, 도구가 20개를 넘으면 등록 안내를 한 줄로 더합니다(D50 결정 8).
///
/// ```
/// use snap_core::agent::{text_protocol_block, ToolRegistry};
/// let registry = ToolRegistry::with_file_tools();
/// let block = text_protocol_block(&registry);
/// assert!(block.contains("read_file"));
/// assert!(block.contains("JSON"));
/// ```
#[must_use]
pub fn text_protocol_block(registry: &ToolRegistry) -> String {
    let mut block = String::new();
    block.push_str(TEXT_PROTOCOL_HEAD);
    block.push('\n');
    for tool in registry.tools() {
        block.push_str(&tool_line(
            tool.name(),
            tool.description(),
            &tool.parameters_schema(),
        ));
        block.push('\n');
    }
    block.push_str(TEXT_PROTOCOL_RULES);
    if registry.tools().len() > TEXT_PROTOCOL_TOOL_GUIDE_LIMIT {
        block.push('\n');
        block.push_str("도구 등록 안내: 한 번에 노출하는 도구는 20개 미만으로 줄여줘.");
    }
    block
}

/// 도구 한 개에 대한 2안 글자 3줄(이름·인자 필드와 타입·호출 예시 JSON)을 만듭니다.
fn tool_line(name: &str, description: &str, schema: &Value) -> String {
    format!(
        "{name}({description}): {}\n예시: {}",
        argument_fields(schema),
        call_example(name, schema),
    )
}

/// 스키마의 `properties`를 "이름:타입(필수)" 목록으로 줄여줍니다(읽기용 짧은 글자).
fn argument_fields(schema: &Value) -> String {
    let properties = properties_in_order(schema);
    if properties.is_empty() {
        return "인자 없음".to_owned();
    }
    let fields: Vec<String> = properties
        .iter()
        .map(|(key, ty, is_required)| {
            let mark = if *is_required { "필수" } else { "선택" };
            format!("{key}:{ty}({mark})")
        })
        .collect();
    fields.join(", ")
}

/// 호출 예시 JSON 글자입니다. 지시문에 "JSON" 글자가 필요하므로 이 자리에 "JSON"을 둡니다.
fn call_example(name: &str, schema: &Value) -> String {
    format!(
        "JSON 형식으로 {{\"name\":\"{name}\",\"arguments\":{}}}",
        example_arguments(schema)
    )
}

/// 예시 인자 객체 글자입니다. `required`에 있는 필드를 앞세우고 자리표시 값을 채웁니다.
fn example_arguments(schema: &Value) -> String {
    let properties = properties_in_order(schema);
    if properties.is_empty() {
        return "{}".to_owned();
    }
    let items: Vec<String> = properties
        .iter()
        .map(|(key, ty, _)| format!("\"{key}\":{}", placeholder(ty, key)))
        .collect();
    format!("{{{}}}", items.join(","))
}

/// 타입에 맞춘 예시 값 글자입니다. 짧은 자리표시만 씁니다.
fn placeholder(ty: &str, key: &str) -> String {
    match ty {
        "number" | "integer" => "0".to_owned(),
        "boolean" => "true".to_owned(),
        "array" => "[]".to_owned(),
        "object" => "{}".to_owned(),
        _ => format!("여기에_{key}"),
    }
}

/// 스키마의 `properties`를 `(이름, 타입, 필수 여부)` 목록으로 뽑습니다.
///
/// `required`에 있는 필드를 앞에 세우고, 그 밖의 필드는 등록 순서대로 뒤에 둡니다.
fn properties_in_order(schema: &Value) -> Vec<(String, String, bool)> {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return Vec::new();
    };
    let required = required_names(schema);
    let take = |is_required: bool| {
        properties
            .iter()
            .filter(|(key, _)| {
                required.iter().any(|item| item.as_str() == key.as_str()) == is_required
            })
            .map(|(key, value)| {
                let ty = value
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or("string")
                    .to_owned();
                (key.clone(), ty, is_required)
            })
            .collect::<Vec<_>>()
    };
    [take(true), take(false)].concat()
}

/// 스키마의 `required` 목록 이름만 뽑습니다.
fn required_names(schema: &Value) -> Vec<String> {
    schema
        .get("required")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}
