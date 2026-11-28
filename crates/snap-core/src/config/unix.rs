//! Linux snap 설정 폴더 기준 경로 결정.
use std::env::var_os;
use std::path::Path;
use std::path::PathBuf;

use crate::config::config_dir_in;

/// Linux에서 snap 설정 폴더 경로. `$XDG_CONFIG_HOME/snap`, 미설정 시 `~/.config/snap`.
pub fn config_dir() -> PathBuf {
    let base = match var_os("XDG_CONFIG_HOME") {
        Some(value) if !value.is_empty() => PathBuf::from(value),
        _ => {
            let home = var_os("HOME").unwrap_or_default();
            Path::new(&home).join(".config")
        }
    };
    config_dir_in(&base)
}
