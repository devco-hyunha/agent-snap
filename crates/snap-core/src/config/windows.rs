//! Windows snap 설정 폴더 경로 결정.

use std::env::var_os;
use std::path::Path;
use std::path::PathBuf;

use crate::consts::defaults::CONFIG_DIR_NAME;

/// Windows에서 snap 설정 폴더 경로. `%APPDATA%\snap`.
pub fn config_dir() -> PathBuf {
    let base = var_os("APPDATA").unwrap_or_else(|| var_os("USERPROFILE").unwrap_or_default());
    config_dir_in(Path::new(&base))
}

/// base 경로를 받아 snap 설정 폴더 경로 결정.
pub fn config_dir_in(base: &Path) -> PathBuf {
    base.join(CONFIG_DIR_NAME)
}
