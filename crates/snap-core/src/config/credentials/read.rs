//! 자격 파일 읽기·해석 자리.

use std::fs::read_to_string;
use std::path::Path;

use serde_json::Map;
use serde_json::Value;

use crate::config::credentials::error::CredentialsError;
use crate::config::credentials::shape::api_keys_from_value;
use crate::consts::defaults::CREDENTIALS_FILE_NAME;
use crate::jsonc::parse_jsonc;

/// 자격 파일을 읽는 자리. 없는 파일은 MissingFile로 잡음.
pub fn read_credentials(config_dir: &Path) -> Result<Map<String, Value>, CredentialsError> {
    let path = credentials_path(config_dir);
    let text = read_to_string(&path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => CredentialsError::MissingFile {
            path: path.display().to_string(),
        },
        _ => CredentialsError::ReadFailed {
            path: path.display().to_string(),
        },
    })?;
    credentials_from_str(&text, &path)
}

/// 자격 파일 내용을 텍스트로 읽는 자리. 경로 표시는 오류 문구용.
pub fn credentials_from_str(
    text: &str,
    path_display: &Path,
) -> Result<Map<String, Value>, CredentialsError> {
    let value = parse_jsonc(text).map_err(|e| CredentialsError::ParseFailed {
        path: path_display.display().to_string(),
        line: e.line(),
    })?;
    api_keys_from_value(&value, path_display)
}

/// 설정 폴더 안 자격 파일 경로.
fn credentials_path(config_dir: &Path) -> std::path::PathBuf {
    config_dir.join(CREDENTIALS_FILE_NAME)
}
