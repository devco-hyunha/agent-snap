//! 한국어 설정 문구.
/// Debug 출력에 쓰는 가렸음 표시.
pub const SECRET_REDACTED: &str = "가려짐";

/// 자격 파일이 없을 때 문구. 경로만 붙인다.
pub const CREDENTIALS_FILE_NOT_FOUND: &str = "자격 파일이 없습니다";

/// 자격 파일을 읽지 못했을 때 문구. 경로만 붙인다.
pub const CREDENTIALS_READ_FAILED: &str = "자격 파일을 읽을 수 없습니다";

/// 자격 파일을 해석하지 못했을 때 문구. 경로만 붙인다.
pub const CREDENTIALS_PARSE_FAILED: &str = "자격 파일 형식을 해석할 수 없습니다";

/// api_keys 구조가 기준과 다를 때 문구. 경로만 붙인다.
pub const CREDENTIALS_TYPE_FAILED: &str = "자격 파일의 api_keys 형식이 올바르지 않습니다";
