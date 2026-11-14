//! 비밀 파일 이름 판정만 맡습니다. 비밀 목록과 `.env` 계열 이름 판별을 담습니다(D17·AC 8, SRP 분할).
//!
//! 거부는 `path_guard`가 정하고, 이 파일의 [`is_secret_name`] 판정만 그 순서에서 부릅니다.
//! 목록·검색 도구도 이 판정을 그대로 재사용합니다(DRY).
//! 이름 비교 관례는 `path_norm`의 [`names_match`]를 따라 OS마다 같은 기준을 맞춥니다.

use std::path::Path;

use crate::config::{CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME};

use super::path_norm::names_match;

/// 마지막 이름이 비밀 파일(`agent-snap.jsonc`·`credentials.jsonc`, `.env` 계열)인지 확인합니다(D17·AC 8).
///
/// 정규화가 끝난 경로의 마지막 이름만 비교하므로 폴더 이름과 섞이지 않습니다.
/// `.env`는 이름이 정확히 `.env`이고, `.env.local`처럼 `.env`로 시작하는 파일도 비밀로 봅니다.
/// 이름 비교는 `names_match` 관례를 따라 Windows는 대소문자를 가리지 않습니다.
///
/// 목록·검색 도구가 이 판정을 재사용합니다(DRY).
pub fn is_secret_name(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    if names_match(name, CONFIG_FILE_NAME) || names_match(name, CREDENTIALS_FILE_NAME) {
        return true;
    }
    is_env_family(name)
}

/// `.env` 계열 이름의 접두어입니다(AC 8, T4).
const ENV_SECRET_PREFIX: &str = ".env";

/// `.env` 계열 이름을 판별합니다(AC 8).
///
/// 이름이 `.env`와 같지 않아도 접두어가 있으면 비밀로 봅니다(예: `.env.local`).
/// Windows는 대소문자를 가리지 않고, 그 밖의 운영체제는 문자 그대로 비교합니다.
fn is_env_family(name: &str) -> bool {
    if cfg!(windows) {
        return name.to_ascii_lowercase().starts_with(ENV_SECRET_PREFIX);
    }
    name.starts_with(ENV_SECRET_PREFIX)
}
