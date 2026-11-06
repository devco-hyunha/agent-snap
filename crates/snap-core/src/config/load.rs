//! 설정 폴더에서 설정 파일과 자격 파일 텍스트를 읽습니다. 해석은 `Config::parse`와
//! `Credentials::parse`가 맡습니다.

use std::io::ErrorKind;
use std::path::Path;

use super::error::{ConfigError, ReadingError};
use super::read::Config;
use crate::credentials::Credentials;

/// 설정 파일 이름입니다.
pub const CONFIG_FILE_NAME: &str = "agent-snap.jsonc";
/// 자격 파일 이름입니다.
pub const CREDENTIALS_FILE_NAME: &str = "credentials.jsonc";

/// 설정 폴더의 `agent-snap.jsonc`를 읽어 검증합니다.
///
/// # 오류
/// 파일을 읽지 못하면 폴더 경로와 파일 이름만 담은 [`ReadingError::Io`], 내용이 틀리면
/// 해석·검증 오류입니다.
pub fn load_config(folder: &Path) -> Result<Config, ConfigError> {
    Config::parse(&read_file(folder, CONFIG_FILE_NAME)?)
}

/// 설정 폴더의 `credentials.jsonc`를 읽습니다. 파일이 없으면 키가 없는 상태를 돌려줍니다.
///
/// # 오류
/// 파일이 없는 경우 외의 읽기 실패는 [`ReadingError::Io`], 내용 오류는 [`ReadingError::Credentials`]입니다.
pub fn load_credentials(folder: &Path) -> Result<Credentials, ConfigError> {
    match read_file(folder, CREDENTIALS_FILE_NAME) {
        Ok(text) => Ok(Credentials::parse(&text)?),
        Err(ConfigError::Reading(ReadingError::Io { source, .. }))
            if source.kind() == ErrorKind::NotFound =>
        {
            Ok(Credentials::absent())
        }
        Err(error) => Err(error),
    }
}

fn read_file(folder: &Path, file: &'static str) -> Result<String, ConfigError> {
    std::fs::read_to_string(folder.join(file)).map_err(|source| {
        ReadingError::Io {
            folder: folder.to_path_buf(),
            file,
            source,
        }
        .into()
    })
}
