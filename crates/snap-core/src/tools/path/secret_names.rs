//! 비밀 파일 이름 판정. 비밀 목록과 `.env` 계열 이름 판별을 담음
//!
//! 목록·검색 도구도 이 판정을 재사용. 이름 비교는 `path_norm`의 [`names_match`]를 따라 OS마다 같은 기준

use std::path::Path;

use crate::config::{CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME};

use super::path_norm::names_match;

/// 마지막 이름이 비밀 파일(`agent-snap.jsonc`·`credentials.jsonc`, `.env` 계열)인지 확인
///
/// 정규화가 끝난 경로의 마지막 이름만 비교하므로 폴더 이름과 섞이지 않음
pub fn is_secret_name(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    if names_match(name, CONFIG_FILE_NAME) || names_match(name, CREDENTIALS_FILE_NAME) {
        return true;
    }
    is_env_family(name)
}

/// `.env` 계열 이름의 접두어
const ENV_SECRET_PREFIX: &str = ".env";

/// `.env` 계열 이름을 판별
///
/// 이름이 `.env`와 같지 않아도 접두어가 있으면 비밀로 봄. Windows는 대소문자를 가리지 않고, 그 밖의 운영체제는 문자 그대로 비교
fn is_env_family(name: &str) -> bool {
    if cfg!(windows) {
        return name.to_ascii_lowercase().starts_with(ENV_SECRET_PREFIX);
    }
    name.starts_with(ENV_SECRET_PREFIX)
}
