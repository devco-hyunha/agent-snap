//! 설정 저장 연산 시험 공통 재료. 시험 폴더에 설정 파일을 놓고 설정 폴더 경로를 돌려주는 조립만 담음.
//! 엔드포인트 연산·모델 연산 시험이 재사용하는 재료 파일.

use std::path::Path;
use std::path::PathBuf;

/// 시험 폴더에 `snap` 설정 폴더를 만들고 `snap.jsonc`를 쓴 뒤 설정 폴더 경로를 돌려줌.
#[allow(clippy::expect_used)]
pub(crate) fn write_config(base: &Path, text: &str) -> PathBuf {
    let config_dir = base.join("snap");
    std::fs::create_dir_all(&config_dir).expect("test creates config folder");
    std::fs::write(config_dir.join("snap.jsonc"), text).expect("test writes config file");
    config_dir
}
