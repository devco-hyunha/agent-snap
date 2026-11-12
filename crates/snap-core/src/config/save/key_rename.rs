//! 자격 파일(`credentials.jsonc`의 `api_keys`) 키 이름을 바꾸고 지우는 편집 자리입니다.
//!
//! 이 파일의 책임은 하나입니다. 자격 파일 쪽 키 이름 바꾸기(값 옮기기)와 키 지우기.
//! 저장이 두 파일을 고칠 때 자격 파일 쪽에서 일어나는 편집이고, 설정 파일 쪽 이름
//! 바꾸기·삭제는 `save_rename`이, 키를 추가·수정하는 저장은 `save_credentials`가 맡습니다.
//! 02-1 해석 검증과 편집 대상 찾기는 `save_credentials`가 정한 것을 그대로 쓰며, 이름
//! 바꾸기가 설정 파일 쪽 이름 변경보다 먼저 끝나야 하는 순서(D40 규칙 3)는 `save_store`가
//! 지킵니다.

use std::path::Path;

use super::super::error::ConfigError;
use super::atomic::{Replace, replace_file};
use super::credentials::{api_keys_of, read_checked_root, set_key, write_root};
use super::cst::{properties_named, remove_named, remove_props, string_value_of};
use super::names::{NameScope, checked_save_name};

/// 이름 바꾸기를 부탁하는 정보입니다. 대조는 trim한 값으로 합니다.
#[derive(Debug)]
pub struct KeyRename {
    /// 바꾸기 전 엔드포인트 이름입니다.
    pub from: String,
    /// 새 엔드포인트 이름입니다.
    pub to: String,
}

/// 자격 파일의 키 이름을 새 이름으로 바꿉니다. 값은 그대로 둡니다.
///
/// 두 이름은 trim한 값으로 대조하고, 자격 파일에 기존 키가 없으면 파일을 건드리지 않고
/// 성공합니다. 새 이름 칸이 이미 있으면 그 칸 값을 옮기는 값으로 채우고, 이름이 겹치는 칸은
/// 남기지 않습니다. 02-1 해석이 값으로 보는 마지막 `from` 칸의 값만 옮깁니다.
///
/// [`SaveError::SaveIo`]: super::super::error::SaveError::SaveIo
///
/// # 오류
/// 파일 읽기·쓰기 실패는 대상 파일 종류(자격)와 오류 종류만 밝히는 [`SaveError::SaveIo`],
/// 이름 규칙 위반은 [`EndpointIssue::EmptyEndpointName`]·[`EndpointIssue::EndpointNameHasSlash`],
/// 02-1 해석이 거부하는 파일은 [`ReadingError::Credentials`]입니다. 기존 키가 없는 경우를
/// 빼면 파일을 건드리기 전에 막습니다.
pub fn rename_key(folder: &Path, rename: &KeyRename) -> Result<(), ConfigError> {
    rename_key_with(folder, rename, &replace_file)
}

/// 위 공개 함수가 쓰는 내부 경로입니다. 교체 지점(`replace`)만 시험용 함수로 바꿀 수 있게
/// 분리했습니다. 이 인자 하나로 두 함수 경로가 한 자리에 모이며, 이름 바꾸기가 설정 파일 쪽
/// 이름 변경보다 먼저 끝나야 하는 순서(D40 규칙 3)는 `rename_endpoint`가 지킵니다.
pub(crate) fn rename_key_with(
    folder: &Path,
    rename: &KeyRename,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let from = checked_save_name(&rename.from, NameScope::Endpoint)?;
    let to = checked_save_name(&rename.to, NameScope::Endpoint)?;
    // 이름이 같으면 값을 바꿀 것도, 파일에 손댈 것도 없습니다.
    if from == to {
        return Ok(());
    }
    let (root, text) = read_checked_root(folder)?;
    let api_keys = api_keys_of(&root)?;
    let froms = properties_named(&api_keys, &from);
    // 옮길 값은 02-1 해석 규칙(같은 이름은 마지막 값)과 같은 마지막 칸에서 꺼냅니다.
    let moved = froms.last().and_then(string_value_of);
    // 값이 문자열이 아닌 칸은 02-1 해석이 먼저 거부하므로 여기까지 오지 않습니다.
    let Some(moved) = moved else {
        return Ok(());
    };
    // `from` 칸은 나온 자리 전부에서 골라 모두 지웁니다. 값을 옮긴 뒤이므로 값 없는 칸이 남지 않습니다.
    remove_props(&froms);
    // 새 이름 칸은 `set_key` 경로로만 더합니다. `set_key`는 같은 이름 칸이 이미 있으면
    // 첫 칸에 값을 쓰고 나머지 칸은 지우므로, 저장 뒤에 남는 `to` 칸은 값 있는 칸 하나입니다.
    set_key(&api_keys, &to, &moved);
    write_root(folder, &root, &text, replace)
}

/// 엔드포인트를 삭제할 때 자격 파일의 그 키를 지웁니다.
///
/// 키가 없으면 파일을 건드리지 않고 성공합니다. 이름이 겹치는 칸이 있으면 그 칸들을 함께
/// 지웁니다.
///
/// [`SaveError::SaveIo`]: super::super::error::SaveError::SaveIo
///
/// # 오류
/// 파일 읽기 실패는 대상 파일 종류(자격)와 오류 종류만 밝히는 [`SaveError::SaveIo`], 이름 규칙
/// 위반은 [`EndpointIssue::EmptyEndpointName`]·[`EndpointIssue::EndpointNameHasSlash`], 02-1 해석이
/// 거부하는 파일은 [`ReadingError::Credentials`]입니다. 파일을 건드리기 전에 막습니다.
pub fn remove_key(folder: &Path, endpoint: &str) -> Result<(), ConfigError> {
    remove_key_with(folder, endpoint, &replace_file)
}

/// 위 공개 함수가 쓰는 내부 경로입니다. 교체 지점(`replace`)만 시험용 함수로 바꿀 수 있게
/// 분리했습니다.
pub(crate) fn remove_key_with(
    folder: &Path,
    endpoint: &str,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let name = checked_save_name(endpoint, NameScope::Endpoint)?;
    let (root, text) = read_checked_root(folder)?;
    let api_keys = api_keys_of(&root)?;
    // 키가 없으면 파일을 건드릴 이유가 없습니다(삭제할 엔드포인트가 없는 저장과 같은 처리).
    let removed = remove_named(&api_keys, &name);
    if removed == 0 {
        return Ok(());
    }
    write_root(folder, &root, &text, replace)
}
