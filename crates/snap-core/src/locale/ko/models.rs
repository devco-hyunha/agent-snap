//! 한국어 모델 목록 문구.

/// 모델 목록 응답에 `data` 배열이 없을 때 문구. 값은 붙이지 않음.
pub const MODELS_DATA_MISSING: &str = "모델 목록 응답에 data 배열이 없습니다";

/// `data` 항목의 `id`가 문자열이 아닐 때 문구. 항목 번호만 붙임.
pub const MODELS_ID_NOT_STRING: &str = "모델 목록 data 항목의 id가 문자열이 아닙니다";
