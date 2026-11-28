//! snap 설정 파일 읽기 자리. 해석·기본값·검증을 순서대로 호출.

use std::fs::read_to_string;
use std::path::Path;

use crate::config::error::ConfigError;
use crate::config::snap_file_in;
use crate::config::types::Config;
use crate::config::validate::check_config;
use crate::config::validate::trim_names;
use crate::jsonc::parse_jsonc;

/// 설정 폴더 안 `snap.jsonc`를 읽는 자리.
pub fn read_config(config_dir: &Path) -> Result<Config, ConfigError> {
    let path = snap_file_in(config_dir);
    let text = read_to_string(&path).map_err(|_| ConfigError::ReadFailed {
        path: path.display().to_string(),
    })?;
    config_from_str(&text, &path)
}

/// 설정 텍스트를 읽는 자리. 검증 규칙을 거친 뒤 타입으로 옮김.
pub fn config_from_str(text: &str, path_display: &Path) -> Result<Config, ConfigError> {
    let mut value = parse_jsonc(text).map_err(|e| ConfigError::ParseFailed {
        path: path_display.display().to_string(),
        line: e.line(),
    })?;
    check_config(&value).map_err(|e| ConfigError::Validation(Box::new(e)))?;
    trim_names(&mut value);
    let config = serde_json::from_value(value).map_err(|_| ConfigError::InvalidShape {
        path: path_display.display().to_string(),
    })?;
    Ok(config)
}
