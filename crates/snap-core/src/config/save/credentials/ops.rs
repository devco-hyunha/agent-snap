//! `credentials.jsonc`의 `api_keys` 표에 키를 추가·수정·삭제하는 자리.
//! 저장은 `save_validated`가 검증하고, 항목을 고르는 규칙은 `select`가 맡는다. 키는 이 파일에만 쓴다.

use jsonc_parser::cst::CstInputValue;
use jsonc_parser::cst::CstObject;
use std::path::Path;

use crate::config::credentials::credentials_path;
use crate::config::save::credentials::check::check_credentials;
use crate::config::save::credentials::read::read_credentials_text;
use crate::config::save::credentials::select::api_keys_object;
use crate::config::save::credentials::select::find_key;
use crate::config::save::error::ConfigSaveError;
use crate::config::save::validated::save_validated;

/// 엔드포인트 이름과 키 값을 받아 `api_keys` 표에 항목을 더한다.
/// 자격 파일이 없으면 최상위 `api_keys` 객체를 가진 파일로 새로 만든다.
pub fn add_key(config_dir: &Path, endpoint: &str, key: &str) -> Result<(), ConfigSaveError> {
    let name = endpoint.trim().to_owned();
    let value = key.trim().to_owned();
    save_credentials(config_dir, move |object| {
        let keys = api_keys_object(object, &credentials_path(config_dir))?;
        keys.append(&name, CstInputValue::from(value));
        Ok(())
    })
}

/// 같은 엔드포인트 이름 항목의 키 값만 바꾼다. 다른 항목은 건드리지 않는다.
pub fn update_key(config_dir: &Path, endpoint: &str, key: &str) -> Result<(), ConfigSaveError> {
    let name = endpoint.trim().to_owned();
    let value = key.trim().to_owned();
    save_credentials(config_dir, move |object| {
        let keys = api_keys_object(object, &credentials_path(config_dir))?;
        let target = find_key(&keys, &name)?;
        target.set_value(CstInputValue::from(value));
        Ok(())
    })
}

/// 같은 엔드포인트 이름 항목을 `api_keys` 표에서 제거한다.
pub fn remove_key(config_dir: &Path, endpoint: &str) -> Result<(), ConfigSaveError> {
    let name = endpoint.trim().to_owned();
    save_credentials(config_dir, move |object| {
        let keys = api_keys_object(object, &credentials_path(config_dir))?;
        let target = find_key(&keys, &name)?;
        target.remove();
        Ok(())
    })
}

/// 자격 파일을 읽고 편집하고, 결과를 자격 구조 검사로 확인한 뒤에만 원자 저장.
fn save_credentials(
    config_dir: &Path,
    edit: impl FnOnce(&CstObject) -> Result<(), ConfigSaveError>,
) -> Result<(), ConfigSaveError> {
    let path = credentials_path(config_dir);
    let check_path = path.clone();
    save_validated(&path, read_credentials_text, edit, move |value| {
        check_credentials(value, &check_path)
    })
}
