//! 모델 목록 응답에서 `data` 배열의 문자열 `id`만 모는 자리. 형식 규칙 한 책임만 담음.
//!
//! `object`·`created`·`owned_by` 등 나머지 필드는 읽지 않는다. 응답 원문은 오류에 담지 않는다.

use serde_json::Value;

use crate::consts::request::DATA_KEY;
use crate::consts::request::ID_KEY;
use crate::llm::client::model::error::ModelsFormatError;

use super::error::ModelsFormatProblem;

/// 모델 목록 응답에서 `data` 배열의 문자열 `id`만 모아 돌려줌.
///
/// `data` 배열이 없거나 항목이 객체가 아니거나 `id`가 문자열이 아니면 형식 오류.
/// 빈 `data` 배열은 빈 목록으로 정상 처리.
pub fn collect_ids(response: &Value) -> Result<Vec<String>, ModelsFormatError> {
    let Some(data) = response.get(DATA_KEY) else {
        return Err(ModelsFormatError::new(ModelsFormatProblem::DataMissing));
    };
    let Some(items) = data.as_array() else {
        return Err(ModelsFormatError::new(ModelsFormatProblem::DataMissing));
    };
    ids_of(items)
}

/// `data` 배열 항목에서 `id` 문자열만 순서대로 모음.
fn ids_of(items: &[Value]) -> Result<Vec<String>, ModelsFormatError> {
    items
        .iter()
        .enumerate()
        .map(|(index, item)| id_of(item, index))
        .collect()
}

/// 항목 하나에서 `id` 문자열만. 없거나 문자열이 아니면 형식 오류.
fn id_of(item: &Value, index: usize) -> Result<String, ModelsFormatError> {
    let id = item.get(ID_KEY).and_then(Value::as_str);
    let Some(id) = id else {
        return Err(ModelsFormatError::new(ModelsFormatProblem::IdNotString {
            index,
        }));
    };
    Ok(id.to_owned())
}
