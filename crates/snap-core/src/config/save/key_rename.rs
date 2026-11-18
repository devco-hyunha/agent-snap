//! 자격 파일(`credentials.jsonc`의 `api_keys`) 키 이름을 바꾸고 지우는 편집

use std::path::Path;

use super::super::error::ConfigError;
use super::atomic::{Replace, replace_file};
use super::credentials::{api_keys_of, read_checked_root, set_key, write_root};
use super::cst::{properties_named, remove_named, remove_props, string_value_of};
use super::names::{NameScope, checked_save_name};

/// 이름 바꾸기 정보. 대조는 trim한 값으로 함
#[derive(Debug)]
pub struct KeyRename {
    /// 바꾸기 전 엔드포인트 이름
    pub from: String,
    /// 새 엔드포인트 이름
    pub to: String,
}

/// 자격 파일의 키 이름을 새 이름으로 바꾸고 값은 그대로 둠
///
/// 두 이름은 trim한 값으로 대조. 기존 키가 없으면 파일을 건드리지 않고 성공. 새 이름 칸이 이미 있으면 그 칸 값을 옮기는 값으로 채우고, 이름이 겹치는 칸은 남기지 않음. 자격 파일 해석이 값으로 보는 마지막 `from` 칸의 값만 옮김
///
/// # 오류
/// 읽기·쓰기 실패는 [`super::super::error::SaveError::SaveIo`], 이름 규칙 위반은 [`super::super::error::EndpointIssue::EmptyEndpointName`]·[`super::super::error::EndpointIssue::EndpointNameHasSlash`], 자격 파일 해석 거부는 [`super::super::error::ReadingError::Credentials`]. 기존 키가 없는 경우를 빼면 파일을 건드리기 전에 막음
pub fn rename_key(folder: &Path, rename: &KeyRename) -> Result<(), ConfigError> {
    rename_key_with(folder, rename, &replace_file)
}

/// 공개 함수가 쓰는 내부 경로. 교체 지점(`replace`)만 시험용으로 바꿀 수 있게 분리
pub(crate) fn rename_key_with(
    folder: &Path,
    rename: &KeyRename,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let from = checked_save_name(&rename.from, NameScope::Endpoint)?;
    let to = checked_save_name(&rename.to, NameScope::Endpoint)?;
    // 이름이 같으면 값을 바꿀 것도, 파일에 손댈 것도 없음
    if from == to {
        return Ok(());
    }
    let (root, text) = read_checked_root(folder)?;
    let api_keys = api_keys_of(&root)?;
    let froms = properties_named(&api_keys, &from);
    // 옮길 값은 자격 파일 해석 규칙(같은 이름은 마지막 값)과 같은 마지막 칸에서 꺼냄
    let moved = froms.last().and_then(string_value_of);
    // 값이 문자열이 아닌 칸은 자격 파일 해석이 먼저 거부하므로 여기까지 오지 않음
    let Some(moved) = moved else {
        return Ok(());
    };
    // `from` 칸은 나온 자리 전부에서 골라 모두 지움. 값을 옮긴 뒤이므로 값 없는 칸이 남지 않음
    remove_props(&froms);
    // 새 이름 칸은 `set_key` 경로로만 더함. `set_key`는 같은 이름 칸이 이미 있으면
    // 첫 칸에 값을 쓰고 나머지 칸은 지우므로, 저장 뒤에 남는 `to` 칸은 값 있는 칸 하나
    set_key(&api_keys, &to, &moved);
    write_root(folder, &root, &text, replace)
}

/// 엔드포인트를 삭제할 때 자격 파일의 그 키를 지움
///
/// 키가 없으면 파일을 건드리지 않고 성공. 이름이 겹치는 칸이 있으면 그 칸들을 함께 지움
///
/// # 오류
/// 읽기 실패는 [`super::super::error::SaveError::SaveIo`], 이름 규칙 위반은 [`super::super::error::EndpointIssue::EmptyEndpointName`]·[`super::super::error::EndpointIssue::EndpointNameHasSlash`], 자격 파일 해석 거부는 [`super::super::error::ReadingError::Credentials`]. 파일을 건드리기 전에 막음
pub fn remove_key(folder: &Path, endpoint: &str) -> Result<(), ConfigError> {
    remove_key_with(folder, endpoint, &replace_file)
}

/// 공개 함수가 쓰는 내부 경로. 교체 지점(`replace`)만 시험용으로 바꿀 수 있게 분리
pub(crate) fn remove_key_with(
    folder: &Path,
    endpoint: &str,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let name = checked_save_name(endpoint, NameScope::Endpoint)?;
    let (root, text) = read_checked_root(folder)?;
    let api_keys = api_keys_of(&root)?;
    // 키가 없으면 파일을 건드릴 이유가 없음(삭제할 엔드포인트가 없는 저장과 같은 처리)
    let removed = remove_named(&api_keys, &name);
    if removed == 0 {
        return Ok(());
    }
    write_root(folder, &root, &text, replace)
}
