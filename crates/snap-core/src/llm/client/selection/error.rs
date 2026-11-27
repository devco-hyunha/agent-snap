//! 선택 키 해석·조회 실패 오류 자리. 키 원문과 주소는 담지 않음.

use thiserror::Error;

use crate::locale::selection::SELECTION_KEY_EMPTY_PART;
use crate::locale::selection::SELECTION_KEY_NO_SEPARATOR;
use crate::locale::selection::SELECTION_TARGET_MISSING;

/// 선택 키 형식 오류의 세부 사실. 원문 값은 담지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SelectionFormatProblem {
    /// 구분자가 아예 없음.
    MissingSeparator,

    /// 구분자는 있으나 앞·뒤가 trim 후 빔.
    EmptyPart,
}

impl std::fmt::Display for SelectionFormatProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::MissingSeparator => SELECTION_KEY_NO_SEPARATOR,
            Self::EmptyPart => SELECTION_KEY_EMPTY_PART,
        };
        f.write_str(text)
    }
}

/// 선택 키 해석·조회 실패. 라이브러리가 쓰는 오류 타입.
#[derive(Debug, Error)]
pub enum SelectionError {
    /// 키 형식 오류. 구분자 없음·빈 칸 사실만 담음.
    #[error("{problem}")]
    Format {
        /// 형식 실패 세부 사실.
        problem: SelectionFormatProblem,
    },

    /// 설정에 대상 없음. 엔드포인트 이름과 모델 이름만 담음.
    #[error("{SELECTION_TARGET_MISSING}: {endpoint} / {model}")]
    TargetMissing {
        /// 찾지 못한 엔드포인트 이름.
        endpoint: String,

        /// 찾지 못한 모델 이름.
        model: String,
    },
}

impl SelectionError {
    /// 형식 오류 만들기. 세부 사실만 씀.
    pub const fn format(problem: SelectionFormatProblem) -> Self {
        Self::Format { problem }
    }

    /// 대상 없음 오류 만들기. 이름 둘만 씀.
    pub fn target_missing(endpoint: &str, model: &str) -> Self {
        Self::TargetMissing {
            endpoint: endpoint.to_owned(),
            model: model.to_owned(),
        }
    }
}
