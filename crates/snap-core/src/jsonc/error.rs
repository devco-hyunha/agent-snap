//! JSONC 해석 실패 상태 자리. 값·줄 내용은 미포함.

use thiserror::Error;

use crate::locale::jsonc::JSONC_PARSE_FAILED;
use crate::locale::jsonc::JSONC_ROOT_NOT_OBJECT;

/// JSONC 해석 실패 상태. 줄 번호만 밝히고 값·줄 내용은 미포함.
#[derive(Debug, Error)]
pub enum JsoncError {
    /// 문법이 틀린 자리. 줄 번호만 밝힘.
    #[error("{JSONC_PARSE_FAILED}: line {line}")]
    Syntax {
        /// 오류가 생긴 줄 번호(1부터 시작).
        line: usize,
    },

    /// 최상위 값이 객체가 아닌 자리. 값은 미포함.
    #[error("{JSONC_ROOT_NOT_OBJECT}")]
    RootNotObject,
}

impl JsoncError {
    /// 오류가 생긴 줄 번호 반환. 없는 자리에는 0.
    pub(crate) fn line(&self) -> usize {
        match self {
            Self::Syntax { line } => *line,
            Self::RootNotObject => 0,
        }
    }
}
