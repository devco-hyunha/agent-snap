//! 한국어 선택 키 문구.

/// 구분자가 없는 선택 키 거부 문구.
pub const SELECTION_KEY_NO_SEPARATOR: &str = "선택 키에 /가 없습니다";

/// 선택 키 앞·뒤가 빈 경우 거부 문구.
pub const SELECTION_KEY_EMPTY_PART: &str =
    "선택 키의 엔드포인트 이름 또는 모델 이름이 비어 있습니다";

/// 선택 대상이 설정에 없을 때 문구. 이름만 붙임.
pub const SELECTION_TARGET_MISSING: &str = "선택한 엔드포인트 또는 모델이 설정에 없습니다";
