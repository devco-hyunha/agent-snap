//! 영어 선택 키 문구.

/// 구분자가 없는 선택 키 거부 문구.
pub const SELECTION_KEY_NO_SEPARATOR: &str = "Selection key has no /";

/// 선택 키 앞·뒤가 빈 경우 거부 문구.
pub const SELECTION_KEY_EMPTY_PART: &str =
    "Endpoint name or model name is empty in the selection key";

/// 선택 대상이 설정에 없을 때 문구. 이름만 붙임.
pub const SELECTION_TARGET_MISSING: &str = "Selected endpoint or model is not in the config";
