//! 설정·자격 파일 저장 무: 검증한 값을 원자적으로 반영합니다.

pub(crate) mod atomic;
mod check;
pub(crate) mod config;
pub(crate) mod credentials;
mod cst;
pub(crate) mod error;
pub(crate) mod io_error;
pub(crate) mod key_rename;
mod names;
mod read;
mod rename;
pub(crate) mod store;
pub(crate) mod temp_file;
mod values;

pub use check::{EndpointSave, ModelSave};
pub use config::save_endpoint;
pub use credentials::save_key;
pub use error::{SaveIoKind, SaveIoTarget};
pub use io_error::{save_io, save_io_target};
pub use key_rename::{KeyRename, remove_key, rename_key};
pub use store::{remove_endpoint, rename_endpoint};
