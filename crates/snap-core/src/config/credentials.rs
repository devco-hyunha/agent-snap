//! 자격 파일 읽기. `credentials.jsonc`를 읽어 api_keys 표에서 키를 찾는다.

use std::fs::read_to_string;
use std::path::Path;

use serde_json::Map;
use serde_json::Value;
use thiserror::Error;

use crate::config::SecretString;
use crate::consts::defaults::API_KEYS_KEY;
use crate::consts::defaults::CREDENTIALS_FILE_NAME;
use crate::locale::config::CREDENTIALS_FILE_NOT_FOUND;
use crate::locale::config::CREDENTIALS_PARSE_FAILED;
use crate::locale::config::CREDENTIALS_READ_FAILED;
use crate::locale::config::CREDENTIALS_TYPE_FAILED;

/// 자격 파일 읽기 실패 상태. 경로는 밝히고 파일 값은 내지 않는다.
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

    /// jsonc 해석 실패 자리.
    #[error("{CREDENTIALS_PARSE_FAILED}: {path}")]
    ParseFailed {
        /// 자격 파일 경로.
        path: String,
    },

    /// api_keys 구조 검증 실패 자리.
    #[error("{CREDENTIALS_TYPE_FAILED}: {path}")]
    InvalidShape {
        /// 자격 파일 경로.
        path: String,
    },
}

/// 자격 파일 파일을 읽는 자리. 없는 파일은 MissingFile로 잡는다.
pub fn read_credentials(config_dir: &Path) -> Result<Map<String, Value>, CredentialsError> {
    let path = credentials_path(config_dir);
    let text = read_to_string(&path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => CredentialsError::MissingFile {
            path: display_path(&path),
        },
        _ => CredentialsError::ReadFailed {
            path: display_path(&path),
        },
    })?;
    credentials_from_str(&text, &path)
}

/// 자격 파일 내용을 텍스트로 읽는 자리. 경로 표시는 오류 문구에 쓴다.
pub fn credentials_from_str(
    text: &str,
    path_display: &Path,
) -> Result<Map<String, Value>, CredentialsError> {
    let stripped = json_comments::StripComments::new(text.as_bytes());
    let value = serde_json::from_reader(stripped).map_err(|_| CredentialsError::ParseFailed {
        path: display_path(path_display),
    })?;
    api_keys_from_value(&value, path_display)
}

/// api_keys 표에서 trim한 엔드포인트 이름으로 키를 찾는다. 없으면 None.
pub fn find_api_key(keys: &Map<String, Value>, endpoint: &str) -> Option<SecretString> {
    let target = endpoint.trim();
    let name = keys.keys().find(|name| name.trim() == target)?;
    let value = keys.get(name)?.as_str()?;
    Some(SecretString::new(value.to_owned()))
}

/// 값에서 api_keys 표만 꺼낸다. 누락·비객체·비문자열 값은 검증 실패로 잡는다.
fn api_keys_from_value(
    value: &Value,
    path_display: &Path,
) -> Result<Map<String, Value>, CredentialsError> {
    let root = value
        .as_object()
        .ok_or_else(|| invalid_shape(path_display))?;
    let keys = root
        .get(API_KEYS_KEY)
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_shape(path_display))?;
    if keys.values().any(|item| item.as_str().is_none()) {
        return Err(invalid_shape(path_display));
    }
    Ok(keys.clone())
}

/// 설정 폴더 안 자격 파일 경로.
fn credentials_path(config_dir: &Path) -> std::path::PathBuf {
    config_dir.join(CREDENTIALS_FILE_NAME)
}

/// 검증 실패를 경로와 함께 만든다.
fn invalid_shape(path_display: &Path) -> CredentialsError {
    CredentialsError::InvalidShape {
        path: display_path(path_display),
    }
}

/// 오류 문구에 쓰는 경로 표시.
fn display_path(path: &Path) -> String {
    path.display().to_string()
}
