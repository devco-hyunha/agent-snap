//! JSONC 저장 실패 상태 자리. 경로만 담고 키·주소·파일 내용은 미포함.

use thiserror::Error;

use crate::locale::save::SAVE_CREATE_FOLDER_FAILED;
use crate::locale::save::SAVE_CREATE_TEMP_FAILED;
use crate::locale::save::SAVE_NO_FOLDER;
use crate::locale::save::SAVE_REPLACE_FAILED;
use crate::locale::save::SAVE_WRITE_TEMP_FAILED;

/// JSONC 저장 실패 상태. 경로만 밝히고 키·주소·파일 내용은 담지 않는다.
#[derive(Debug, Error)]
pub enum SaveError {
    /// 저장 경로에 폴더 칸이 없는 자리. 경로만 밝힘.
    #[error("{SAVE_NO_FOLDER}: {path}")]
    NoFolder {
        /// 저장하려는 파일 경로.
        path: String,
    },

    /// 설정 폴더를 만들지 못한 자리. 폴더 경로만 밝힘.
    #[error("{SAVE_CREATE_FOLDER_FAILED}: {path}")]
    CreateFolder {
        /// 만들지 못한 폴더 경로.
        path: String,
    },

    /// 임시 파일을 만들지 못한 자리. 폴더 경로만 밝힘.
    #[error("{SAVE_CREATE_TEMP_FAILED}: {path}")]
    CreateTemp {
        /// 임시 파일을 만들지 못한 폴더 경로.
        path: String,
    },

    /// 임시 파일에 쓰지 못한 자리. 폴더 경로만 밝힘.
    #[error("{SAVE_WRITE_TEMP_FAILED}: {path}")]
    WriteTemp {
        /// 임시 파일이 있던 폴더 경로.
        path: String,
    },

    /// 원본 교체를 하지 못한 자리. 원본 경로만 밝힘. 원본은 그대로다.
    #[error("{SAVE_REPLACE_FAILED}: {path}")]
    Replace {
        /// 교체를 하지 못한 원본 경로.
        path: String,
    },
}
