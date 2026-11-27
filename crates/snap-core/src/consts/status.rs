//! 상태 오류 본문 읽기 상한 자리. 전송이 본문에서 읽는 최대 크기만 담음.

use crate::llm::SUMMARY_MAX_CHARS;

/// 상태 오류 본문 읽기 상한 바이트. 요약 상한보다 넉넉하게 읽음.
pub const BODY_MAX_BYTES: usize = SUMMARY_MAX_CHARS * 4;
