//! JSONC 해석 자리. 파서 옵션의 한곳 정본.

use jsonc_parser::ParseOptions;
use serde_json::Value;

use crate::jsonc::error::JsoncError;

/// 해석 규칙의 한곳 정본. 주석은 허용, 마지막 쉼표와 느슨한 문법은 거부.
pub(crate) fn parse_options() -> ParseOptions {
    ParseOptions {
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
    }
}

/// JSONC 텍스트를 serde 값으로 읽는 자리.
pub fn parse_jsonc(text: &str) -> Result<Value, JsoncError> {
    jsonc_parser::parse_to_serde_value(text, &parse_options()).map_err(|e| JsoncError::Syntax {
        line: e.line_display(),
    })
}
