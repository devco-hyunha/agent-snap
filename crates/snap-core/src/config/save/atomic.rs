//! 설정 파일·자격 파일의 원자적 교체: 같은 폴더 임시 파일에 전체를 쓰고, 반영한 뒤 기존 파일을 바꿈
//!
//! 저장이 교체 지점을 함수로 받아 교체 실패 시험이 자기 파일 이름으로 교체를 막을 수 있게 함.

use std::path::Path;

use super::super::error::{ConfigError, SaveError};
use super::error::REPLACE;
use super::io_error::{save_io_kind, save_io_target};
use super::temp_file::{temp_name, write_temp};

/// 파일 교체를 맡기는 함수 타입. `from`에서 `target`으로 옮기는 동작만 담음.
///
/// 교체 실패 시험이 같은 폴더 안의 시험용 파일 이름으로 교체를 막을 수 있게 이 타입을 받음.
pub(crate) type Replace = dyn Fn(&Path, &Path) -> Result<(), std::io::Error>;

/// 표준 교체 지점. `std::fs::rename`은 같은 폴더의 파일만 바꿈.
pub(crate) fn replace_file(from: &Path, target: &Path) -> Result<(), std::io::Error> {
    std::fs::rename(from, target)
}

/// `text`를 같은 폴더의 임시 파일에 쓰고, 디스크에 반영한 뒤 `replace`로 기존 `file`을 바꿈.
///
/// 교체에 실패하면 기존 파일은 그대로 두고 이번 저장이 만든 임시 파일만 지움.
///
/// # 오류
/// 임시 파일 쓰기·교체 실패는 [`SaveError::SaveIo`]로 대상 파일 종류와 오류 종류만 밝힘. 파일 경로·키 값·base_url은 밝히지 않음.
///
/// [`SaveError::SaveIo`]: super::super::error::SaveError::SaveIo
pub(crate) fn write_replacing(
    folder: &Path,
    file: &'static str,
    text: &str,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let temp_path = folder.join(temp_name(file));
    // 임시 파일부터 만들어야 교체 실패 뒤에 지울 임시 파일 경로가 정해짐.
    write_temp(&temp_path, text, file)?;
    if let Err(source) = replace(&temp_path, &folder.join(file)) {
        // 교체 실패는 기존 파일을 건드리지 않고, 이번 저장이 만든 임시 파일만 지움.
        let _ = std::fs::remove_file(&temp_path);
        // 원인은 파일 경로가 섞여 있어 담지 않음. 종류만 정함.
        return Err(SaveError::SaveIo {
            target: save_io_target(file),
            kind: save_io_kind(REPLACE, &source),
        }
        .into());
    }
    Ok(())
}
