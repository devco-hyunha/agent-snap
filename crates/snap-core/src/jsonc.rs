//! JSONC 해석 공용 자리. 파서 오류는 줄 번호만 담는 오류로 묶음.

use jsonc_parser::ParseOptions;
use serde_json::Value;
use thiserror::Error;

use crate::locale::jsonc::JSONC_PARSE_FAILED;

/// JSONC 해석 실패 상태. 줄 번호만 밝히고 값·줄 내용은 미포함.
#[derive(Debug, Error)]
pub enum JsoncError {
    /// 문법이 틀린 자리. 줄 번호만 밝힘.
    #[error("{JSONC_PARSE_FAILED}: line {line}")]
    Syntax {
        /// 오류가 생긴 줄 번호(1부터 시작).
        line: usize,
    },
}

impl JsoncError {
    /// 오류가 생긴 줄 번호 반환. 크레이트 안에서만 쓰는 범위.
    pub(crate) fn line(&self) -> usize {
        match self {
            Self::Syntax { line } => *line,
        }
    }
}

/// JSONC 텍스트를 serde 값으로 읽는 자리. 주석은 허용, 마지막 쉼표는 거부.
pub fn parse_jsonc(text: &str) -> Result<Value, JsoncError> {
    let options = ParseOptions {
        allow_comments: true,
        allow_loose_object_property_names: false,
        allow_trailing_commas: false,
        allow_missing_commas: false,
        allow_single_quoted_strings: false,
        allow_hexadecimal_numbers: false,
        allow_unary_plus_numbers: false,
        allow_bare_decimal_point_numbers: false,
        allow_non_finite_numbers: false,
        allow_extended_string_escapes: false,
    };
    jsonc_parser::parse_to_serde_value(text, &options).map_err(|e| JsoncError::Syntax {
        line: e.line_display(),
    })
}
