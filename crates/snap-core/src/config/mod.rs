//! 설정 읽기·저장 모듈 자리. OS별 파일은 cfg 한 줄로만 여는 구조.
use std::path::Path;
use std::path::PathBuf;

use crate::consts::defaults::CONFIG_DIR_NAME;

pub mod credentials;
mod defaults;
mod error;
pub mod keys;
pub mod reader;
pub mod secret_string;
pub mod types;
pub mod validate;

pub use credentials::CredentialsError;
pub use credentials::credentials_from_str;
pub use credentials::find_api_key;
pub use credentials::read_credentials;
pub use error::ConfigError;
pub use keys::resolve_api_key;
pub use reader::config_from_str;
pub use reader::read_config;
pub use secret_string::SecretString;
pub use types::Config;
pub use types::EndpointConfig;
pub use types::HttpSettings;
pub use types::ModelConfig;
pub use types::Sampling;
pub use validate::ValidationError;

#[cfg(windows)]
pub use windows::config_dir;
#[cfg(windows)]
mod windows;

#[cfg(all(unix, not(windows)))]
pub use unix::config_dir;
#[cfg(all(unix, not(windows)))]
mod unix;

/// base 경로를 받아 snap 설정 폴더 경로 결정. OS와 무관한 값의 한곳 정본.
pub fn config_dir_in(base: &Path) -> PathBuf {
    base.join(CONFIG_DIR_NAME)
}
