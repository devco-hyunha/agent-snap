//! 설정 값(`ExtraValue`·실수)을 JSON 값으로 바꿉니다.
//!
//! 요청 본문에 무엇을 넣을지는 호출 쪽이 정하고, 여기는 형태만 맞춥니다.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::config::ExtraValue;

/// `ExtraValue`를 `serde_json::Value`로 바꿉니다.
pub(crate) fn extra_to_json(value: &ExtraValue) -> Value {
    match value {
        ExtraValue::Null => Value::Null,
        ExtraValue::Bool(flag) => Value::Bool(*flag),
        ExtraValue::Number(text) => parse_number_text(text),
        ExtraValue::String(text) => Value::String(text.clone()),
        ExtraValue::Array(items) => Value::Array(items.iter().map(extra_to_json).collect()),
        ExtraValue::Object(object) => object_to_json(object),
    }
}

/// 유한한 실수를 JSON 숫자로 바꿉니다. 유한하지 않으면 `null`입니다.
pub(crate) fn json_f64(value: f64) -> Value {
    serde_json::Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

fn object_to_json(object: &BTreeMap<String, ExtraValue>) -> Value {
    let mut map = Map::new();
    for (key, child) in object {
        map.insert(key.clone(), extra_to_json(child));
    }
    Value::Object(map)
}

/// 설정에서 숫자 문자열로 보관한 값을 JSON 숫자로 되돌립니다.
fn parse_number_text(text: &str) -> Value {
    match serde_json::from_str::<Value>(text) {
        Ok(value @ Value::Number(_)) => value,
        _ => Value::String(text.to_owned()),
    }
}

/// 설정의 추가 값을 요청 본문 맵에 합칩니다(03-6 R2 감사 분리: `llm/request_body.rs`에서 옮김).
///
/// ExtraValue→JSON 지식은 이 파일의 [`extra_to_json`]만 쓰고, 합치기 자리도 한 곳으로 모읍니다(DRY).
pub(crate) fn merge_extra(map: &mut Map<String, Value>, extra: &BTreeMap<String, ExtraValue>) {
    for (key, value) in extra {
        map.insert(key.clone(), extra_to_json(value));
    }
}
