//! 자격 파일(`credentials.jsonc`의 `api_keys` 아래 엔드포인트 이름별 키를 쓰는 모듈

use std::path::Path;

use jsonc_parser::ParseOptions;
use jsonc_parser::cst::{CstInputValue, CstObject, CstRootNode};

use super::super::error::{ConfigError, ReadingError};
use super::super::load::CREDENTIALS_FILE_NAME;
use super::atomic::{Replace, replace_file, write_replacing};
use super::cst::{properties_named, remove_props, set_string};
use super::names::{NameScope, checked_save_name};
use super::read::read_text;
use crate::credentials::{Credentials, CredentialsError};
use crate::secrets::SecretValue;

/// 자격 파일의 `api_keys`에 엔드포인트 이름으로 키를 추가하고, 같은 이름이 있으면 값을 교체
///
/// 검증(이름 규칙·자격 파일 해석)에 걸리면 파일을 건드리지 않음. CST 편집이라 주석과 손대지 않은 항목은 원문대로 남음, 파일이 없으면 새로 만듦. 이름이 겹치는 칸이 있으면 첫 칸에 값을 쓰고 나머지를 지워, 해석이 읽는 값과 저장 값을 늘 같게 유지
///
/// # 오류
/// 읽기·쓰기 실패는 [`super::super::error::SaveError::SaveIo`], 이름 규칙 위반은 [`super::super::error::EndpointIssue::EmptyEndpointName`]·[`super::super::error::EndpointIssue::EndpointNameHasSlash`], 자격 파일 해석 거부는 [`ReadingError::Credentials`]
pub fn save_key(folder: &Path, endpoint: &str, key: &SecretValue) -> Result<(), ConfigError> {
    save_key_with(folder, endpoint, key, &replace_file)
}

/// 공개 함수가 쓰는 내부 경로. 교체 지점(`replace`)만 시험용으로 바꿀 수 있게 분리
pub(crate) fn save_key_with(
    folder: &Path,
    endpoint: &str,
    key: &SecretValue,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let name = checked_save_name(endpoint, NameScope::Endpoint)?;
    let (root, text) = read_checked_root(folder)?;
    let api_keys = api_keys_of(&root)?;
    set_key(&api_keys, &name, key.reveal());
    write_root(folder, &root, &text, replace)
}

/// 자격 파일 텍스트를 읽어 해석을 통과시킨 뒤 CST(원문 트리)와 원문을 함께 반환
///
/// 해석이 통과한 텍스트만 편집 대상으로 삼음. 거부 파일은 이 단계에서 걸러져 편집 경로가 없음. 파일이 없으면 빈 텍스트로 시작(새 파일)
pub(crate) fn read_checked_root(folder: &Path) -> Result<(CstRootNode, String), ConfigError> {
    let text = read_text(folder, CREDENTIALS_FILE_NAME)?;
    // 빈 텍스트는 새 파일(파일 없음) 자리이므로 해석 검증을 거치지 않음
    // 그 밖의 텍스트는 자격 파일 해석을 검증으로 재사용. 키 값은 오류에 미포함
    if !text.trim().is_empty() {
        Credentials::parse(&text).map_err(ConfigError::from)?;
    }
    // 해석이 통과한 텍스트이므로 CST 해석도 같은 입력에서 실패하지 않음
    let root = CstRootNode::parse(&text, &ParseOptions::default())
        .map_err(|_| ReadingError::Credentials(CredentialsError::RootNotObject))?;
    Ok((root, text))
}

/// CST 루트의 `api_keys` 객체를 편집 대상으로 찾음. 칸이 없으면 빈 객체를 새로 만듦
///
/// `api_keys`가 객체가 아닌 파일은 해석 단계가 먼저 거부하므로 여기는 빈 칸 자리만 다룸
pub(crate) fn api_keys_of(root: &CstRootNode) -> Result<CstObject, ConfigError> {
    Ok(root
        .object_value_or_create()
        .ok_or(ReadingError::Credentials(CredentialsError::RootNotObject))?
        .object_value_or_create("api_keys")
        .ok_or(ReadingError::Credentials(
            CredentialsError::ApiKeysNotObject,
        ))?)
}

/// `api_keys`의 `endpoint` 이름 칸을 찾아 값을 바꾸고, 없으면 더함
///
/// 이름이 겹치는 칸이 있으면 값 있는 칸 하나만 남김. 값 쓴 칸은 그 자리에 두고 남은 칸만 지워 주석과 칸 자리를 그대로 유지
pub(crate) fn set_key(api_keys: &CstObject, endpoint: &str, value: &str) {
    let named = properties_named(api_keys, endpoint);
    let Some((first, extras)) = named.split_first() else {
        set_string(api_keys, endpoint, value);
        return;
    };
    // 남은 칸들의 값이 서로 다를 수 있어 값을 쓴 칸 하나만 남김
    first.set_value(CstInputValue::String(value.to_owned()));
    remove_props(extras);
}

/// CST를 텍스트로 되돌려 자격 파일에 씀. 원문과 같은 텍스트로 재직렬화되면 파일을 건드리지 않음
///
/// 쓰기는 원자적 교체. 교체 실패는 기존 파일을 그대로 두고 임시 파일만 정리하며, [`super::super::error::SaveError::SaveIo`]로 표시
pub(crate) fn write_root(
    folder: &Path,
    root: &CstRootNode,
    original: &str,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let text = root.to_string();
    if text == original {
        return Ok(());
    }
    write_replacing(folder, CREDENTIALS_FILE_NAME, &text, replace)
}
