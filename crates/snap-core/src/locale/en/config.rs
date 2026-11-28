//! 영어 설정 읽기 문구.
/// Debug 출력에 쓰는 가렸음 표시.
pub const SECRET_REDACTED: &str = "redacted";

/// 설정 파일을 읽지 못했을 때 문구. 경로만 붙임.
pub const CONFIG_READ_FAILED: &str = "Failed to read config file";

/// 설정 파일을 해석하지 못했을 때 문구. 경로와 줄 번호만 붙임.
pub const CONFIG_PARSE_FAILED: &str = "Failed to parse config file format";

/// 설정 구조가 기준과 다를 때 문구. 경로만 붙임.
pub const CONFIG_INVALID_SHAPE: &str = "Invalid config file format";
