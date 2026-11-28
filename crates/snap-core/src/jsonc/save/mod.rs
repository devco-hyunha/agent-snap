//! 원자 저장 자리. 임시 파일을 원본 폴더에 쓰고 원본과 교체.
pub(crate) mod error;

use std::fs::create_dir_all;
use std::io::Write as _;
use std::path::Path;

use crate::jsonc::save::error::SaveError;

/// 텍스트 원자 저장. 폴더가 없으면 만들고 원본 폴더에 임시 파일을 쓴 뒤 교체.
/// 교체 실패 시 임시 파일만 사라짐: 원본은 그대로. copy 대체 경로 없음.
pub fn write_text_atomic(path: &Path, text: &str) -> Result<(), SaveError> {
    // 파일 이름만 있는 경로는 저장할 폴더가 없음: 거절로 처리.
    let Some(folder) = path
        .parent()
        .filter(|folder| !folder.as_os_str().is_empty())
    else {
        return Err(SaveError::NoFolder {
            path: path.display().to_string(),
        });
    };
    create_folder(folder)?;
    write_temp_then_replace(path, folder, text)
}

/// 설정 폴더 생성. 이미 있으면 그대로 둠.
fn create_folder(folder: &Path) -> Result<(), SaveError> {
    if folder.is_dir() {
        return Ok(());
    }
    create_dir_all(folder).map_err(|_| SaveError::CreateFolder {
        path: folder.display().to_string(),
    })
}

/// 임시 파일에 쓰고 원본과 교체. 교체 실패 시 임시 파일은 드롭으로 정리됨.
fn write_temp_then_replace(path: &Path, folder: &Path, text: &str) -> Result<(), SaveError> {
    let folder_display = || folder.display().to_string();
    let mut temp = tempfile::NamedTempFile::new_in(folder).map_err(|_| SaveError::CreateTemp {
        path: folder_display(),
    })?;
    let write_result = temp.write_all(text.as_bytes()).and_then(|()| temp.flush());
    write_result.map_err(|_| SaveError::WriteTemp {
        path: folder_display(),
    })?;
    temp.persist(path).map_err(|_| SaveError::Replace {
        path: path.display().to_string(),
    })?;
    Ok(())
}
