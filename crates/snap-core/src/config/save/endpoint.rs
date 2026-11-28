//! `snap.jsonc`의 `endpoints` 배열에 엔드포인트를 추가·수정·삭제하는 자리.
//! 저장은 `save_config`가 검증하고, 항목을 고르는 규칙은 `select`가 맡는다. 키 칸은 만들지 않는다.

use jsonc_parser::cst::CstInputValue;
use std::path::Path;

use crate::config::save::error::ConfigSaveError;
use crate::config::save::keys::BASE_URL_KEY;
use crate::config::save::keys::KIND_KEY;
use crate::config::save::keys::NAME_KEY;
use crate::config::save::select::endpoints_array;
use crate::config::save::select::find_endpoint;
use crate::config::save::select::find_endpoint_object;
use crate::config::save::select::replace_fields;
use crate::config::save::validated::save_config;
use crate::config::snap_file_in;

/// 엔드포인트 이름·주소·`kind`를 받아 `endpoints` 배열 끝에 항목을 더한다.
pub fn add_endpoint(
    config_dir: &Path,
    name: &str,
    base_url: &str,
    kind: &str,
) -> Result<(), ConfigSaveError> {
    let fields = endpoint_fields(name, base_url, kind);
    let error_path = snap_file_in(config_dir);
    save_config(config_dir, move |object| {
        let array = endpoints_array(object, &error_path)?;
        array.append(CstInputValue::Object(fields));
        Ok(())
    })
}

/// 같은 이름 항목의 주소·`kind`만 바꾼다. 다른 항목은 건드리지 않는다.
pub fn update_endpoint(
    config_dir: &Path,
    name: &str,
    base_url: &str,
    kind: &str,
) -> Result<(), ConfigSaveError> {
    let fields = endpoint_fields(name, base_url, kind);
    let target_name = name.trim().to_owned();
    let error_path = snap_file_in(config_dir);
    save_config(config_dir, move |object| {
        let array = endpoints_array(object, &error_path)?;
        let target = find_endpoint_object(&array, &target_name)?;
        replace_fields(&target, &fields);
        Ok(())
    })
}

/// 같은 이름 항목을 배열에서 제거한다.
pub fn remove_endpoint(config_dir: &Path, name: &str) -> Result<(), ConfigSaveError> {
    let target_name = name.trim().to_owned();
    let error_path = snap_file_in(config_dir);
    save_config(config_dir, move |object| {
        let array = endpoints_array(object, &error_path)?;
        let target = find_endpoint(&array, &target_name)?;
        target.remove();
        Ok(())
    })
}

/// 추가·수정이 만드는 항목의 칸들. 키 칸은 없다.
fn endpoint_fields(name: &str, base_url: &str, kind: &str) -> Vec<(String, CstInputValue)> {
    [(NAME_KEY, name), (BASE_URL_KEY, base_url), (KIND_KEY, kind)]
        .map(|(key, value)| (key.to_owned(), CstInputValue::from(value.trim())))
        .to_vec()
}
