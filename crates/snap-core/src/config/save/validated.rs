//! 편집 결과를 검증한 뒤에만 저장하는 골격 자리. 검증 규칙은 받은 창구가 정한다.

use std::cell::RefCell;
use std::fs::read_to_string;
use std::path::Path;

use jsonc_parser::cst::CstObject;
use serde_json::Value;

use crate::config::save::error::ConfigSaveError;
use crate::config::snap_file_in;
use crate::config::validate::check_config;
use crate::jsonc::edit_jsonc;
use crate::jsonc::parse_jsonc;
use crate::jsonc::write_text_atomic;

/// 파일을 읽고 편집하고, 결과를 다시 해석해 검사한 뒤에만 원자 저장.
/// `read`은 파일 텍스트 창구, `check`은 해석 결과 검사 창구. 두 창구가 정본 규칙을 부른다.
/// 검사에 걸리면 저장이 타지 않는다. 편집이 실패하면 파일을 건드리지 않는다.
pub(crate) fn save_validated(
    path: &Path,
    read: impl FnOnce(&Path) -> Result<String, ConfigSaveError>,
    edit: impl FnOnce(&CstObject) -> Result<(), ConfigSaveError>,
    check: impl FnOnce(&Value) -> Result<(), ConfigSaveError>,
) -> Result<(), ConfigSaveError> {
    let text = read(path)?;
    let edit_failure = RefCell::new(None);
    let edited = edit_jsonc(&text, |object| {
        let slot = &edit_failure;
        if let Err(error) = edit(object) {
            slot.replace(Some(error));
        }
    })
    .map_err(|e| ConfigSaveError::Jsonc(Box::new(e)))?;
    let failure = edit_failure.take();
    if let Some(error) = failure {
        return Err(error);
    }
    let value = parse_jsonc(&edited).map_err(|e| ConfigSaveError::Jsonc(Box::new(e)))?;
    check(&value)?;
    write_text_atomic(path, &edited).map_err(|e| ConfigSaveError::Save(Box::new(e)))
}

/// `snap.jsonc`를 설정 검증(`check_config`)으로 확인한 뒤에만 저장하는 자리.
pub(crate) fn save_config(
    config_dir: &Path,
    edit: impl FnOnce(&CstObject) -> Result<(), ConfigSaveError>,
) -> Result<(), ConfigSaveError> {
    let path = snap_file_in(config_dir);
    save_validated(&path, read_config_text, edit, |value| {
        check_config(value).map_err(|e| ConfigSaveError::Validation(Box::new(e)))
    })
}

/// `snap.jsonc` 텍스트 읽기. 파일이 없으면 읽기 실패로 밝힌다.
fn read_config_text(path: &Path) -> Result<String, ConfigSaveError> {
    read_to_string(path).map_err(|_| ConfigSaveError::Read {
        path: path.display().to_string(),
    })
}
