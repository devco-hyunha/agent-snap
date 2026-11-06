//! CST 편집의 값 변환(`ExtraValue`·`Sampling`을 CST 값으로)과 값 꺼내기: 저장 두 쪽이 함께 쓰는 자리입니다.
//!
//! 이 파일의 책임은 하나입니다. CST 편집이 받는 값으로 바꾸는 것. CST 트리를 찾고 바꾸는 자리는
//! `save_cst`에 두고, 요청 본문 JSON으로 바꾸는 자리는 `extra_json`에 둡니다(dev-principles 2절 SRP).
//! 숫자는 읽을 때 보관한 적힌 모양을 그대로 쓰므로 원본을 잃지 않습니다.

use std::collections::BTreeMap;

use jsonc_parser::cst::CstInputValue;

use super::super::read::types::{ExtraValue, Sampling};

/// 준 항목만 담은 `sampling` 값입니다. 항목이 하나도 없으면 `None`(칸을 안 씀).
pub(super) fn sampling_value(sampling: Sampling) -> Option<CstInputValue> {
    let items: Vec<(String, CstInputValue)> = [
        ("temperature", sampling.temperature.map(CstInputValue::from)),
        ("top_p", sampling.top_p.map(CstInputValue::from)),
        ("max_tokens", sampling.max_tokens.map(CstInputValue::from)),
    ]
    .into_iter()
    .filter_map(|(key, value)| value.map(|value| (key.to_owned(), value)))
    .collect();
    (!items.is_empty()).then_some(CstInputValue::Object(items))
}

/// 준 값이 있을 때만 만드는 `extra` 값입니다. 비어 있으면 `None`이라 칸을 쓰지 않습니다.
pub(super) fn extra_value(extra: &BTreeMap<String, ExtraValue>) -> Option<CstInputValue> {
    (!extra.is_empty()).then(|| CstInputValue::Object(extra_to_cst(extra)))
}

/// `ExtraValue` 맵을 CST 편집이 받는 객체 값으로 바꿉니다.
fn extra_to_cst(extra: &BTreeMap<String, ExtraValue>) -> Vec<(String, CstInputValue)> {
    extra
        .iter()
        .map(|(key, value)| (key.clone(), extra_to_cst_value(value)))
        .collect()
}

/// `ExtraValue` 하나를 CST 편집이 받는 값으로 바꿉니다. 숫자는 읽을 때 보관한 적힌 모양을 씁니다.
fn extra_to_cst_value(value: &ExtraValue) -> CstInputValue {
    match value {
        ExtraValue::Null => CstInputValue::Null,
        ExtraValue::Bool(flag) => CstInputValue::Bool(*flag),
        ExtraValue::Number(text) => CstInputValue::Number(text.clone()),
        ExtraValue::String(text) => CstInputValue::String(text.clone()),
        ExtraValue::Array(items) => {
            CstInputValue::Array(items.iter().map(extra_to_cst_value).collect())
        }
        ExtraValue::Object(object) => CstInputValue::Object(extra_to_cst(object)),
    }
}
