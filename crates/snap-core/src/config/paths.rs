//! 설정 경로 조립 자리. `snap.jsonc`·설정 폴더 경로를 만드는 함수만 맡는다.

use std::path::Path;
use std::path::PathBuf;

use crate::consts::defaults::CONFIG_DIR_NAME;
use crate::consts::defaults::SNAP_FILE_NAME;

/// base 경로를 받아 snap 설정 폴더 경로 결정. OS와 무관한 값의 한곳 정본.
pub fn config_dir_in(base: &Path) -> PathBuf {
    base.join(CONFIG_DIR_NAME)
}

/// 설정 폴더 경로를 받아 `snap.jsonc` 파일 경로 결정. 경로 조립의 한곳 정본.
pub fn snap_file_in(config_dir: &Path) -> PathBuf {
    config_dir.join(SNAP_FILE_NAME)
}
