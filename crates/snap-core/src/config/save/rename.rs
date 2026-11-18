//! 설정 파일(`agent-snap.jsonc`)의 엔드포인트 이름을 바꾸고 지우는 편집

use std::path::Path;

use jsonc_parser::cst::{CstArray, CstRootNode};

use super::super::error::ConfigError;
use super::super::load::CONFIG_FILE_NAME;
use super::super::read::types::Config;
use super::atomic::{Replace, write_replacing};
use super::config::read_root;
use super::cst::{find_named, set_string};
use super::names::{NameScope, checked_save_name};

/// 이름 바꾸기 편집 결과를 메모리에 먼저 만들고 검증까지 통과한 설정 파일 텍스트를 만듦
///
/// 검증은 자격 파일을 쓰기 전에 마치는 순서 앞쪽. 텍스트를 만들 때 결과를 해석(`Config::parse`)으로 확인하므로, 새 이름 항목이 이미 있어 생기는 이름 중복 거부는 자격 파일을 쓰기 전에 남. 이름이 같으면 바꿀 것이 없어 `None` 반환
///
/// # 오류
/// 이름 규칙 위반과 검증 거부, 파일 읽기 실패는 설정 읽기와 같은 오류이고, 바꿀 엔드포인트가 없으면 [`super::super::error::SaveError::MissingEndpoint`]. 어느 쪽이든 파일을 건드리지 않음
pub(crate) fn prepared_rename_text(
    folder: &Path,
    from: &str,
    to: &str,
) -> Result<Option<String>, ConfigError> {
    // 이름 바꾸기 저장이 넘기는 trim한 이름을 그대로 씀
    let from = checked_save_name(from, NameScope::Endpoint)?;
    let to = checked_save_name(to, NameScope::Endpoint)?;
    if from == to {
        return Ok(None);
    }
    let root = read_root(folder)?;
    let Some(endpoints) = endpoints_of(&root) else {
        return Err(ConfigError::missing_endpoint(&from));
    };
    let Some(object) = find_named(&endpoints.elements(), &from).and_then(|node| node.as_object())
    else {
        return Err(ConfigError::missing_endpoint(&from));
    };
    set_string(&object, "name", &to);
    let text = root.to_string();
    Config::parse(&text)?;
    Ok(Some(text))
}

/// 삭제 편집 결과를 메모리에 먼저 만들고 설정 파일 텍스트를 만듦
///
/// 항목을 지우는 편집은 검증 규칙이 새 거부 값을 만들지 않으므로 검증할 새 규칙이 없음. 이름을 여기서 확인하므로 이름 규칙 거부는 자격 파일을 쓰기 전에 남
///
/// # 오류
/// 이름 규칙 위반은 설정 읽기와 같은 오류이고, 파일 읽기 실패는 저장 오류로 밝힘. 바꿀 엔드포인트가 설정 파일에 없으면 [`super::super::error::SaveError::MissingEndpoint`]로 이름만 밝힘. 어느 쪽이든 파일을 건드리지 않음
pub(crate) fn prepared_remove_text(folder: &Path, name: &str) -> Result<String, ConfigError> {
    let name = checked_save_name(name, NameScope::Endpoint)?;
    let root = read_root(folder)?;
    let Some(endpoints) = endpoints_of(&root) else {
        return Err(ConfigError::missing_endpoint(&name));
    };
    let Some(node) = find_named(&endpoints.elements(), &name) else {
        return Err(ConfigError::missing_endpoint(&name));
    };
    node.remove();
    Ok(root.to_string())
}

/// 검증이 끝난 설정 파일 텍스트를 설정 파일에 교체. 순서 뒤쪽
///
/// `None`이면 파일을 건드리지 않고 성공. 검증은 `prepared_rename_text`·`prepared_remove_text`에서 마쳤으므로, 여기서는 교체 실패만 남
///
/// # 오류
/// 교체 실패는 대상 파일 종류(설정)와 오류 종류만 밝히는 [`super::super::error::SaveError::SaveIo`]
pub(crate) fn write_checked_text(
    folder: &Path,
    text: Option<String>,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let Some(text) = text else { return Ok(()) };
    write_replacing(folder, CONFIG_FILE_NAME, &text, replace)
}

/// CST 루트의 `endpoints` 배열을 편집 대상으로 찾음. 객체가 아니거나 `endpoints` 칸이 배열이 아니면 편집할 항목이 없는 것과 같은 처리로 `None` 반환
fn endpoints_of(root: &CstRootNode) -> Option<CstArray> {
    root.object_value_or_create()?
        .array_value_or_create("endpoints")
}
