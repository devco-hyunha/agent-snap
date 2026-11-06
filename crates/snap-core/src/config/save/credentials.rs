//! 자격 파일(`credentials.jsonc`의 `api_keys` 아래 엔드포인트 이름별 키를 쓰는 모듈입니다.
//!
//! CST(원문 구조를 주석·공백까지 보관하는 트리) 편집을 쓰므로 사용자가 단 주석과 다른 항목이
//! 원문 그대로 남습니다(D40). 파일이 없으면 새 파일로 시작합니다.
//!
//! 쓰기 전에 02-1 해석(`Credentials::parse`)을 먼저 통과시키므로, 해석이 거부하는 파일은
//! 파일을 건드리기 전에 거부됩니다. 키 값은 [`SecretValue`] 래퍼로만 받고, 오류·로그·출력문에
//! 값을 남기지 않습니다. 키 이름은 trim한 값(적힌 그대로)으로 쓰며 대소문자를 갈라 봅니다(D10).
//!
//! 키 이름 바꾸기와 그 지우기는 `save_key_rename`이 각각 맡습니다.
//!
//! 오류의 종류와 담는 것은 부분 열거형별로 정하며, 읽기 쪽은 `error_reading`, 이름 규칙
//! 거부는 `error_validation`, 쓰기 실패는 `save_atomic`이 각각 정한 것을 그대로 넘깁니다.

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

/// 자격 파일의 `api_keys`에 엔드포인트 이름으로 키를 추가하고, 같은 이름이 있으면 값을 바꿉니다.
///
/// 키 값은 [`SecretValue`] 래퍼로만 받고 파일에 씁니다. 주석과 이번 저장이 손대지 않은 항목은
/// 원문 그대로 남고, 파일이 없으면 새로 만듭니다. 엔드포인트 이름 규칙과 02-1 해석을 먼저
/// 통과하므로, 규칙에 맞지 않으면 파일을 건드리지 않습니다. 이름이 겹치는 칸이 있으면 첫 칸에
/// 값을 쓰고 나머지를 지워, 02-1 해석이 이 칸 하나로 읽는 값과 저장 값이 늘 같게 만듭니다.
///
/// [`SaveError::SaveIo`]: super::super::error::SaveError::SaveIo
///
/// # 오류
/// 파일 읽기·쓰기 실패는 대상 파일 종류(자격)와 오류 종류만 밝히는 [`SaveError::SaveIo`],
/// 이름 규칙 위반은 [`ValidationError::EmptyEndpointName`]·[`ValidationError::EndpointNameHasSlash`],
/// 02-1 해석이 거부하는 파일은 [`ReadingError::Credentials`]입니다. 어느 쪽이든 파일을
/// 건드리기 전에 막습니다.
pub fn save_key(folder: &Path, endpoint: &str, key: &SecretValue) -> Result<(), ConfigError> {
    save_key_with(folder, endpoint, key, &replace_file)
}

/// 위 공개 함수가 쓰는 내부 경로입니다. 교체 지점(`replace`)만 시험용 함수로 바꿀 수 있게
/// 분리했습니다. 교체 실패 시험은 이 인자만 넘기면 됩니다.
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

/// 자격 파일 텍스트를 읽어 02-1 해석까지 통과시킨 뒤 CST(원문 트리)와 원문을 함께 돌려줍니다.
///
/// 해석이 통과한 텍스트만 편집 대상으로 삼으므로, 02-1 해석이 거부하는 파일은 이 단계에서
/// 걸러져 편집 경로가 생기지 않습니다. 파일이 없으면 빈 텍스트로 시작합니다(새 파일).
pub(crate) fn read_checked_root(folder: &Path) -> Result<(CstRootNode, String), ConfigError> {
    let text = read_text(folder, CREDENTIALS_FILE_NAME)?;
    // 빈 텍스트는 새 파일(파일 없음)을 뜻하는 자리이므로 해석 검증을 거치지 않습니다.
    // 그 밖의 텍스트는 02-1 해석을 검증으로 재사용합니다(DRY). 키 값은 오류에 담기지 않습니다.
    if !text.trim().is_empty() {
        Credentials::parse(&text).map_err(ConfigError::from)?;
    }
    // 해석이 통과한 텍스트이므로 CST 해석도 같은 입력을 받으면 실패하지 않습니다.
    let root = CstRootNode::parse(&text, &ParseOptions::default())
        .map_err(|_| ReadingError::Credentials(CredentialsError::RootNotObject))?;
    Ok((root, text))
}

/// CST 루트의 `api_keys` 객체를 편집 대상으로 찾습니다. 칸이 없으면 빈 객체를 새로 만들어
/// 줍니다. `api_keys`가 객체가 아닌 파일은 02-1 해석이 먼저 거부하므로 여기는 빈 칸 자리만
/// 다룹니다.
pub(crate) fn api_keys_of(root: &CstRootNode) -> Result<CstObject, ConfigError> {
    Ok(root
        .object_value_or_create()
        .ok_or(ReadingError::Credentials(CredentialsError::RootNotObject))?
        .object_value_or_create("api_keys")
        .ok_or(ReadingError::Credentials(
            CredentialsError::ApiKeysNotObject,
        ))?)
}

/// `api_keys`의 `endpoint` 이름 칸을 찾아 값을 바꾸고, 없으면 더합니다.
///
/// 이름이 겹치는 칸(사용자가 직접 만든 파일)이 있으면 02-1 왕복 규칙이 값 하나로 읽히는
/// 상태로 만들기 위해 값 있는 칸 하나만 남깁니다. 값 쓴 칸을 그 자리에 두고 남은 칸만
/// 지우므로 주석과 칸 자리가 그대로 남습니다.
pub(crate) fn set_key(api_keys: &CstObject, endpoint: &str, value: &str) {
    let named = properties_named(api_keys, endpoint);
    let Some((first, extras)) = named.split_first() else {
        set_string(api_keys, endpoint, value);
        return;
    };
    // 남은 칸들의 값이 서로 다를 수 있어, 값을 쓴 칸 하나만 남기는 것이 안전합니다.
    first.set_value(CstInputValue::String(value.to_owned()));
    remove_props(extras);
}

/// CST를 텍스트로 되돌려 자격 파일에 씁니다. 원문과 같은 텍스트로 재직렬화되면 파일을
/// 건드리지 않습니다(CST가 원문 표기를 그대로 두는 편집인지 확인하는 자리).
///
/// 쓰기는 `save_atomic`의 원자적 교체(D40 규칙 2)를 씁니다. 교체 실패는 기존 파일을 그대로
/// 두고 임시 파일만 정리하며, 대상 파일 종류(자격)와 오류 종류만 밝히는
/// [`SaveError::SaveIo`]로 나타납니다.
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
