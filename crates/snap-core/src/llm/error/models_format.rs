//! 모델 목록 조회의 형식 오류 종류입니다(02-7 작업 3).
//!
//! Display·Debug에 본문 원문·API 키·base_url 호스트를 담지 않고 종류만 담는다(spec 51행).

use std::fmt;

/// `GET /models` 목록 응답의 형식 오류 종류입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelsListFormatKind {
    /// 응답 본문을 JSON으로 읽지 못했습니다.
    InvalidJson,
    /// 최상위에 `data`가 없습니다.
    DataMissing,
    /// `data`가 배열이 아닙니다.
    DataNotArray,
    /// `data` 항목에 `id`가 없습니다.
    IdMissing,
    /// `data` 항목의 `id`가 문자열이 아닙니다.
    IdNotString,
}

impl fmt::Display for ModelsListFormatKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::InvalidJson => "잘못된 JSON",
            Self::DataMissing => "data 없음",
            Self::DataNotArray => "data가 배열 아님",
            Self::IdMissing => "id 없음",
            Self::IdNotString => "id가 문자열 아님",
        };
        f.write_str(name)
    }
}

/// `GET /models` 목록 응답의 형식 오류입니다. 종류만 담고 본문 원문·키·주소는 담지 않습니다.
#[derive(thiserror::Error, Debug)]
#[error("모델 목록 형식 오류: {kind}")]
pub struct ModelsListFormatError {
    /// 형식 오류 종류입니다. 본문 원문은 이 자리에 남지 않습니다.
    pub kind: ModelsListFormatKind,
}
