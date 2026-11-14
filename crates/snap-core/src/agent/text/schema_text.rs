//! 2안 지시 블록이 스키마에서 뽑는 글자만 만드는 자리입니다(03-4 spec AC 2).
//!
//! 블록을 조립하는 일은 `protocol.rs`의 책임이고, 이 파일은 도구 한 개당 3줄(이름·인자
//! 필드·호출 예시)을 채울 때 쓰는 스키마 읽기만 맡습니다. 스키마 읽기는 등록 규칙이나 도구
//! 인자 형식이 바뀒을 때 함께 고쳐야 하므로, 블록 조립과 변경 이유가 다릅니다
//! (dev-principles 2절 SRP).

use serde_json::Value;

/// 도구 한 개에 대한 2안 글자 3줄(이름·인자 필드와 타입·호출 예시 JSON)을 만듭니다.
pub(super) fn tool_line(name: &str, description: &str, schema: &Value) -> String {
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
