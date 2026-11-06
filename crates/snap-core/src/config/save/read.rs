//! 저장이 읽는 설정 파일·자격 파일 텍스트: 읽기 하나만 맡는 자리입니다.
//!
//! 설정 쪽과 자격 쪽이 같은 읽기 규칙을 두 곳에 나눠 적지 않게 모았습니다
//! (dev-principles 3절 DRY).

use std::io::ErrorKind;
use std::path::Path;

use super::super::error::ConfigError;
use super::error::{READ, save_io, save_io_kind};

/// 저장이 파일을 읽어 텍스트를 얻습니다. 파일이 없으면 빈 텍스트(새 파일)로 봅니다.
///
/// 읽기에 실패하면 원인으로 파일 경로를 밝히는 항목은 담지 않습니다(02-8 spec 표 2번째 줄).
/// 대상 파일 종류와 오류 종류만 밝히는 [`SaveError::SaveIo`]로 나타냅니다.
///
/// [`SaveError::SaveIo`]: super::super::error::SaveError::SaveIo
pub(super) fn read_text(folder: &Path, file: &'static str) -> Result<String, ConfigError> {
    match std::fs::read_to_string(folder.join(file)) {
        Ok(text) => Ok(text),
        Err(source) if source.kind() == ErrorKind::NotFound => Ok(String::new()),
        Err(source) => Err(save_io(file, save_io_kind(READ, &source))),
    }
}
