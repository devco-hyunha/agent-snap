//! 설정 읽기 실패 상태 자리. 값·주소·키는 미포함.

use thiserror::Error;

use crate::config::validate::ValidationError;
use crate::locale::config::CONFIG_INVALID_SHAPE;
use crate::locale::config::CONFIG_PARSE_FAILED;
use crate::locale::config::CONFIG_READ_FAILED;

/// 설정 읽기 실패 상태. 값·주소·키는 미포함.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// 설정 파일을 읽지 못한 자리. 경로만 밝힘.
    #[error("{CONFIG_READ_FAILED}: {path}")]
    ReadFailed {
        /// 설정 파일 경로.
        path: String,
    },

    /// 설정 파일 해석 실패 자리. 줄 번호만 밝히고 값·줄 내용은 미포함.
    #[error("{CONFIG_PARSE_FAILED}: {path} line {line}")]
    ParseFailed {
        /// 설정 파일 경로.
        path: String,
        /// 오류가 생긴 줄 번호(1부터 시작).
        line: usize,
    },

    /// 설정 구조가 기준과 다를 자리. 종류만 밝히고 값은 미포함.
    #[error("{CONFIG_INVALID_SHAPE}: {path}")]
    InvalidShape {
        /// 설정 파일 경로.
        path: String,
    },

    /// 검증 규칙에 걸린 자리. 이름만 밝히고 값·주소·키는 미포함.
    #[error(transparent)]
    Validation(
        /// 검증 규칙이 낸 오류. 이름만 담음.
        Box<ValidationError>,
    ),
}
