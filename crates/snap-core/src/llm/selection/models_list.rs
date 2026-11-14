//! `GET /models` 목록 JSON 해석 책임입니다(02-7 작업 2·3).
//!
//! 최상위 `data` 배열의 각 항목에서 문자열 `id`만 필수로 읽고, 나머지 필드(`object`,
//! `created`, `owned_by`, `meta` 등)는 모르는 필드로 받아들이고 버린다(spec·RESEARCH 02 절).
//! 항목의 `meta`가 `null`이어도 무시한다(llama.cpp 실측). 형식 오류 종류는 목록 전용 타입
//! `ModelsListFormatError`로 알리고 본문 원문은 담지 않는다(spec 51행).
//! 모델 목록 해석은 선택·엔드포인트 책임과 이어지므로 `llm/selection/`에 둡니다(03-6 R2 감사 분리).

use crate::llm::error::models_format::{ModelsListFormatError, ModelsListFormatKind};

/// 목록 형식 오류를 종류 한 칸으로 봉지하는 만들기입니다. 본문 원문은 같이 담지 않습니다.
pub(in crate::llm) fn new_format_error(kind: ModelsListFormatKind) -> ModelsListFormatError {
    ModelsListFormatError { kind }
}

/// `data` 배열 항목에서 꺼낸 서버용 id 목록입니다. 목록 해석이 성공했을 때만 채운다.
#[derive(Debug)]
pub(in crate::llm) struct ModelsListIds(pub Vec<String>);

/// 본문이 비면 JSON으로 읽지 못한 것으로 같은 종류를 쓴다(빈 목록 성공 금지, spec 38행).
pub(in crate::llm) fn new_empty_body_error() -> ModelsListFormatError {
    new_format_error(ModelsListFormatKind::InvalidJson)
}

/// `data` 배열 항목에서 문자열 `id`만 꺼내 순서대로 모은다. 항목이 하나라도
/// 정한 형식이 아니면 그 자리에서 목록을 채우지 않고 형식 종류만 알린다(spec 38~39행).
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
