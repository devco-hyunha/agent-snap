//! 설정 값(`ExtraValue`·실수)을 JSON 값으로 변경.
//!
//! 요청 본문에 무엇을 넣을지는 호출 쪽이 정하고, 여기는 형태만 맞춤.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::config::ExtraValue;

/// `ExtraValue`를 `serde_json::Value`로 변경.
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

/// 유한한 실수를 JSON 숫자로 변경. 유한하지 않으면 `null`.
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

/// 설정에서 숫자 문자열로 보관한 값을 JSON 숫자로 복원.
fn parse_number_text(text: &str) -> Value {
    match serde_json::from_str::<Value>(text) {
        Ok(value @ Value::Number(_)) => value,
        _ => Value::String(text.to_owned()),
    }
}

/// 설정의 추가 값을 요청 본문 맵에 병합.
///
/// ExtraValue→JSON 지식은 이 파일의 [`extra_to_json`]만 쓰고, 합치기 자리도 한 곳.
pub(crate) fn merge_extra(map: &mut Map<String, Value>, extra: &BTreeMap<String, ExtraValue>) {
    for (key, value) in extra {
        map.insert(key.clone(), extra_to_json(value));
    }
}
