//! 모델 목록 형식 오류 타입 자리. 필드와 표시 구현만 담음. 응답 원문·키·주소는 미포함.

use thiserror::Error;

use crate::llm::TransportError;
use crate::locale::models::MODELS_DATA_MISSING;
use crate::locale::models::MODELS_ID_NOT_STRING;

/// 모델 목록 형식 오류의 세부 사실. 응답 원문은 담지 않음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelsFormatProblem {
    /// 최상위에 `data` 배열이 없음.
    DataMissing,

    /// `data` 항목의 `id`가 없거나 문자열이 아님. 항목 번호만 담음.
    IdNotString {
        /// 형식 오류가 생긴 `data` 항목 번호(0부터 시작).
        index: usize,
    },
}

impl std::fmt::Display for ModelsFormatProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataMissing => f.write_str(MODELS_DATA_MISSING),
            Self::IdNotString { index } => write!(f, "{MODELS_ID_NOT_STRING}: {index}"),
        }
    }
}

/// 모델 목록 응답 형식 오류. 라이브러리가 쓰는 오류 타입.
#[derive(Debug, Error)]
#[error("{problem}")]
pub struct ModelsFormatError {
    /// 형식 실패 세부 사실.
    problem: ModelsFormatProblem,
}

impl ModelsFormatError {
    /// 세부 사실만으로 형식 오류 만들기.
    pub const fn new(problem: ModelsFormatProblem) -> Self {
        Self { problem }
    }

    /// 형식 실패 세부 사실.
    pub const fn problem(&self) -> ModelsFormatProblem {
        self.problem
    }
}

/// 모델 목록 조회 실패. 형식 오류와 02-5 전송 오류만 담음.
#[derive(Debug, Error)]
pub enum ModelsError {
    /// 응답 형식 오류.
    #[error(transparent)]
    Format(#[from] ModelsFormatError),

    /// 02-5 전송 오류. 상태 코드와 가린 요약만 담김.
    #[error(transparent)]
    Transport(#[from] TransportError),
}
