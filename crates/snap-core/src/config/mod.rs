//! 설정 읽기·저장 모듈 자리. 선언·재수출 골격만 둔다. OS별 파일은 cfg 한 줄로만 여는 구조.
pub mod credentials;
mod defaults;
mod error;
pub mod keys;
mod paths;
pub mod reader;
pub mod save;
pub mod secret_string;
pub mod types;
pub mod validate;

pub use credentials::CredentialsError;
pub use credentials::credentials_from_str;
pub use credentials::find_api_key;
pub use credentials::read_credentials;
pub use error::ConfigError;
pub use keys::resolve_api_key;
pub use paths::config_dir_in;
pub use paths::snap_file_in;
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
