//! `ExtraValue`·실수 → JSON 변환 단위 테스트입니다.

use crate::config::ExtraValue;
use crate::config::extra_json::{extra_to_json, json_f64};
use serde_json::json;

#[test]
fn extra_number_becomes_json_number() {
    assert_eq!(extra_to_json(&ExtraValue::Number("0".to_owned())), json!(0));
    assert_eq!(
        extra_to_json(&ExtraValue::Number("1.5".to_owned())),
        json!(1.5)
    );
}

#[test]
fn extra_number_falls_back_to_string_when_not_a_number() {
    assert_eq!(
        extra_to_json(&ExtraValue::Number("not-a-number".to_owned())),
        json!("not-a-number")
    );
}

#[test]
fn extra_to_json_walks_nested_values() {
    let value = ExtraValue::Object(
        [("keep_alive".to_owned(), ExtraValue::Number("0".to_owned()))]
            .into_iter()
            .collect(),
    );
    assert_eq!(extra_to_json(&value), json!({ "keep_alive": 0 }));
}

#[test]
fn json_f64_encodes_finite_floats() {
    assert_eq!(json_f64(0.2), json!(0.2));
    assert!(json_f64(f64::NAN).is_null());
}
