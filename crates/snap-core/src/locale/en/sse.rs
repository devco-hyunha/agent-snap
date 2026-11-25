//! 영어 SSE 델타 복원 문구.
/// 청크 JSON 해석에 실패했을 때 문구. 위치만 붙임.
pub const CHUNK_PARSE_FAILED: &str = "Failed to parse chunk JSON";

/// 청크 JSON 해석 실패 종류 표시. JSON 문법 오류.
pub const CHUNK_ERROR_SYNTAX: &str = "syntax";

/// 청크 JSON 해석 실패 종류 표시. 값 타입 오류.
pub const CHUNK_ERROR_DATA: &str = "data";

/// 청크 JSON 해석 실패 종류 표시. 읽기 실패·예상 못한 끝.
pub const CHUNK_ERROR_IO_OR_EOF: &str = "io-or-eof";

/// 최상위 `error` 청크로 스트림이 끝났을 때 문구. 가린 요약만 붙임.
pub const STREAM_ERROR_FAILED: &str = "Stream ended by server error";

/// 잘못된 JSON 청크·잘린 마지막 이벤트로 스트림이 끝났을 때 문구. 이벤트 번호와 종류만 붙임.
pub const FORMAT_ERROR_FAILED: &str = "Invalid event format";
