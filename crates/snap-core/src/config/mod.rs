//! 설정 읽기·저장 모듈 자리. OS별 파일은 cfg 한 줄로만 문다.
pub mod credentials;
pub mod secret_string;

pub use credentials::CredentialsError;
pub use credentials::credentials_from_str;
pub use credentials::find_api_key;
pub use credentials::read_credentials;
pub use secret_string::SecretString;

#[cfg(windows)]
pub use windows::config_dir;
#[cfg(windows)]
pub use windows::config_dir_in;
#[cfg(windows)]
mod windows;

#[cfg(all(unix, not(windows)))]
pub use unix::config_dir;
#[cfg(all(unix, not(windows)))]
pub use unix::config_dir_in;
#[cfg(all(unix, not(windows)))]
mod unix;
