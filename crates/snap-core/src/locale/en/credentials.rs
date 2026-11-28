//! 영어 자격 파일 문구.
/// 자격 파일이 없을 때 문구. 경로만 붙임.
pub const CREDENTIALS_FILE_NOT_FOUND: &str = "Credentials file not found";

/// 자격 파일을 읽지 못했을 때 문구. 경로만 붙임.
pub const CREDENTIALS_READ_FAILED: &str = "Failed to read credentials file";

/// 자격 파일을 해석하지 못했을 때 문구. 경로만 붙임.
pub const CREDENTIALS_PARSE_FAILED: &str = "Failed to parse credentials file format";

/// api_keys 구조가 기준과 다를 때 문구. 경로만 붙임.
pub const CREDENTIALS_TYPE_FAILED: &str = "Invalid api_keys format in credentials file";
