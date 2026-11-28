//! 한국어 설정 읽기 문구.
/// Debug 출력에 쓰는 가렸음 표시.
pub const SECRET_REDACTED: &str = "가려짐";

/// 설정 파일을 읽지 못했을 때 문구. 경로만 붙임.
pub const CONFIG_READ_FAILED: &str = "설정 파일을 읽을 수 없습니다";

/// 설정 파일을 해석하지 못했을 때 문구. 경로와 줄 번호만 붙임.
pub const CONFIG_PARSE_FAILED: &str = "설정 파일 형식을 해석할 수 없습니다";

/// 설정 구조가 기준과 다를 때 문구. 경로만 붙임.
pub const CONFIG_INVALID_SHAPE: &str = "설정 파일 형식이 올바르지 않습니다";
