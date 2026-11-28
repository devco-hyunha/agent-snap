//! 자격 키 연산 시험 공통 재료. 시험 폴더에 자격 파일과 설정 파일을 놓고 설정 폴더 경로를 돌려주는 조립만 담음.

use std::path::Path;
use std::path::PathBuf;

/// 자격 파일 이름.
const CREDENTIALS_FILE: &str = "credentials.jsonc";

/// 시험 폴더에 `snap` 설정 폴더를 만들고 자격 파일을 쓴 뒤 설정 폴더 경로를 돌려줌.
#[allow(clippy::expect_used)]
pub(crate) fn write_credentials(base: &Path, text: &str) -> PathBuf {
    let config_dir = base.join("snap");
    std::fs::create_dir_all(&config_dir).expect("test creates config folder");
    std::fs::write(config_dir.join(CREDENTIALS_FILE), text).expect("test writes credentials file");
    config_dir
}

/// 자격 파일과 `snap.jsonc`를 함께 놓고 설정 폴더 경로를 돌려줌. 설정 파일 무변경 확인용.
#[allow(clippy::expect_used)]
pub(crate) fn write_credentials_and_config(
    base: &Path,
    credentials: &str,
    config: &str,
) -> PathBuf {
    let config_dir = write_credentials(base, credentials);
    std::fs::write(config_dir.join("snap.jsonc"), config).expect("test writes config file");
    config_dir
}
