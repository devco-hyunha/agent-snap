//! 한국어 JSONC 해석 문구.
/// JSONC 해석에 실패했을 때 문구. 줄 번호만 붙임.
pub const JSONC_PARSE_FAILED: &str = "JSONC를 해석할 수 없습니다";

/// 최상위 값이 객체가 아닐 때 문구. 값은 붙이지 않는다.
pub const JSONC_ROOT_NOT_OBJECT: &str = "JSONC의 최상위 값은 객체여야 합니다";
