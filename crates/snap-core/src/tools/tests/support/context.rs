//! 시험용 실행 판 재료: 임시 폴더를 작업 폴더로 정한 실행 판

use crate::agent::DEFAULT_TOOL_RESULT_MAX_CHARS;
use crate::tools::tool::ToolContext;

use super::temp_folder::TempFolder;

/// 임시 폴더를 작업 폴더로 정한 실행 판. 잘림 상한은 정본 값 사용
#[must_use]
pub fn context_of(folder: &TempFolder) -> ToolContext {
    ToolContext::new(folder.path(), DEFAULT_TOOL_RESULT_MAX_CHARS)
}
