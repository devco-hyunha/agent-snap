//! 잘린 UTF-8 문자열 오류 타입 자리. 조각 경계에서 잘린 문자열만 밝힘.

use std::str::Utf8Error;

use thiserror::Error;

/// 잘린 UTF-8 문자열 자리. 조각 경계에서 잘린 문자열을 밝힘.
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
#[error(transparent)]
pub struct SplitError(
    /// 잘린 위치와 길이를 밝히는 표준 UTF-8 오류. 값·주소는 미포함.
    #[from]
    Utf8Error,
);
