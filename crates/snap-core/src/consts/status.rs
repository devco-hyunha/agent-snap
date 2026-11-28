//! 상태 오류가 읽는 값 자리. 본문 상한과 헤더 이름을 담음.

use crate::llm::SUMMARY_MAX_CHARS;

/// 상태 오류 본문 읽기 상한 바이트. 요약 상한보다 넉넉하게 읽음.
pub const BODY_MAX_BYTES: usize = SUMMARY_MAX_CHARS * 4;

/// 재시도 대기 권유 응답 헤더 이름. 한 번만 읽는 칸.
pub const RETRY_AFTER_HEADER: &str = "retry-after";
