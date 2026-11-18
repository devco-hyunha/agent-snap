//! `GET /models` 목록 JSON 해석.
//!
//! `data` 배열 항목의 문자열 `id`만 필수. 나머지 필드·`null`은 무시. 형식 오류는 종류만 알리고 본문 원문은 담지 않음.

use crate::llm::error::models_format::{ModelsListFormatError, ModelsListFormatKind};

/// 목록 형식 오류를 종류 한 칸으로 만듦. 본문 원문은 담지 않음.
pub(in crate::llm) fn new_format_error(kind: ModelsListFormatKind) -> ModelsListFormatError {
    ModelsListFormatError { kind }
}

/// `data` 배열 항목에서 꺼낸 서버용 id 목록. 해석 성공 시에만 채움.
#[derive(Debug)]
pub(in crate::llm) struct ModelsListIds(pub Vec<String>);

/// 본문이 비면 JSON으로 읽지 못한 것과 같은 종류. 빈 목록 성공 금지.
pub(in crate::llm) fn new_empty_body_error() -> ModelsListFormatError {
    new_format_error(ModelsListFormatKind::InvalidJson)
}

/// `data` 배열에서 문자열 `id`만 순서대로 모음. 형식 위반이면 목록을 채우지 않고 종류만 알림.
pub(in crate::llm) fn parse_models_ids(json: &str) -> Result<ModelsListIds, ModelsListFormatError> {
    let value: serde_json::Value = serde_json::from_str(json)
        .map_err(|_| new_format_error(ModelsListFormatKind::InvalidJson))?;
    let items = match value.get("data") {
        Some(serde_json::Value::Array(items)) => items,
        Some(_) => return Err(new_format_error(ModelsListFormatKind::DataNotArray)),
        None => return Err(new_format_error(ModelsListFormatKind::DataMissing)),
    };
    let mut ids = Vec::with_capacity(items.len());
    for item in items {
        match item.get("id") {
            Some(serde_json::Value::String(id)) => ids.push(id.clone()),
            Some(serde_json::Value::Null) | None => {
                return Err(new_format_error(ModelsListFormatKind::IdMissing));
            }
            _ => return Err(new_format_error(ModelsListFormatKind::IdNotString)),
        }
    }
    Ok(ModelsListIds(ids))
}
