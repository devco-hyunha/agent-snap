//! 자격 파일 읽기 실패 상태 자리. 경로는 밝히고 파일 값은 미포함.

use thiserror::Error;

use crate::locale::credentials::CREDENTIALS_FILE_NOT_FOUND;
use crate::locale::credentials::CREDENTIALS_PARSE_FAILED;
use crate::locale::credentials::CREDENTIALS_READ_FAILED;
use crate::locale::credentials::CREDENTIALS_TYPE_FAILED;

/// 자격 파일 읽기 실패 상태.
#[derive(Debug, Error)]
pub enum CredentialsError {
    /// 자격 파일이 없는 자리.
    #[error("{CREDENTIALS_FILE_NOT_FOUND}: {path}")]
    MissingFile {
        /// 자격 파일 경로.
        path: String,
    },

    /// 자격 파일 읽기 실패 자리.
    #[error("{CREDENTIALS_READ_FAILED}: {path}")]
    ReadFailed {
        /// 자격 파일 경로.
        path: String,
    },

    /// jsonc 해석 실패 자리. 줄 번호만 밝히고 값·줄 내용은 미포함.
    #[error("{CREDENTIALS_PARSE_FAILED}: {path} line {line}")]
    ParseFailed {
        /// 자격 파일 경로.
        path: String,
        /// 오류가 생긴 줄 번호(1부터 시작).
        line: usize,
    },

    /// api_keys 구조 검증 실패 자리.
    #[error("{CREDENTIALS_TYPE_FAILED}: {path}")]
    InvalidShape {
        /// 자격 파일 경로.
        path: String,
    },
}
