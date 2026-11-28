//! `snap.jsonc`의 `endpoints[].models` 배열에 모델을 추가·수정·삭제하는 자리.
//! 저장은 `save_config`가 검증하고, 항목을 고르는 규칙은 `select`가 맡는다.

use jsonc_parser::cst::CstInputValue;
use std::path::Path;

use crate::config::save::error::ConfigSaveError;
use crate::config::save::keys::CONTEXT_WINDOW_KEY;
use crate::config::save::keys::ID_KEY;
use crate::config::save::keys::NAME_KEY;
use crate::config::save::keys::REASONING_KEY;
use crate::config::save::model::changes::ModelChanges;
use crate::config::save::select::endpoints_array;
use crate::config::save::select::find_endpoint_object;
use crate::config::save::select::find_model;
use crate::config::save::select::find_model_object;
use crate::config::save::select::models_array;
use crate::config::save::select::replace_fields;
use crate::config::save::validated::save_config;
use crate::config::snap_file_in;

/// 엔드포인트 이름·모델 이름·`id`·`context_window`를 받아 그 엔드포인트의 `models` 배열 끝에 항목을 더한다.
/// `context_window`가 `None`이면 칸을 만들지 않고 기본값에 맡긴다. 값 검증은 저장 직전 `check_config`가 맡는다.
pub fn add_model(
    config_dir: &Path,
    endpoint_name: &str,
    name: &str,
    id: &str,
    context_window: Option<u64>,
) -> Result<(), ConfigSaveError> {
    let fields = model_fields(name, id, context_window);
    let endpoint = endpoint_name.trim().to_owned();
    let error_path = snap_file_in(config_dir);
    save_config(config_dir, move |object| {
        let endpoints = endpoints_array(object, &error_path)?;
        let target = find_endpoint_object(&endpoints, &endpoint)?;
        let models = models_array(&target, &endpoint)?;
        models.append(CstInputValue::Object(fields));
        Ok(())
    })
}

/// 같은 엔드포인트 안 같은 이름 모델의 `changes`에 담긴 칸만 바꾼다. 다른 항목은 건드리지 않는다.
pub fn update_model(
    config_dir: &Path,
    endpoint_name: &str,
    model_name: &str,
    changes: &ModelChanges,
) -> Result<(), ConfigSaveError> {
    let fields = change_fields(changes);
    let endpoint = endpoint_name.trim().to_owned();
    let model = model_name.trim().to_owned();
    let error_path = snap_file_in(config_dir);
    save_config(config_dir, move |object| {
        let endpoints = endpoints_array(object, &error_path)?;
        let target = find_endpoint_object(&endpoints, &endpoint)?;
        let models = models_array(&target, &endpoint)?;
        let found = find_model_object(&models, &model)?;
        replace_fields(&found, &fields);
        Ok(())
    })
}

/// 같은 엔드포인트 안 같은 이름 모델 항목을 `models` 배열에서 제거한다.
pub fn remove_model(
    config_dir: &Path,
    endpoint_name: &str,
    model_name: &str,
) -> Result<(), ConfigSaveError> {
    let endpoint = endpoint_name.trim().to_owned();
    let model = model_name.trim().to_owned();
    let error_path = snap_file_in(config_dir);
    save_config(config_dir, move |object| {
        let endpoints = endpoints_array(object, &error_path)?;
        let target = find_endpoint_object(&endpoints, &endpoint)?;
        let models = models_array(&target, &endpoint)?;
        let found = find_model(&models, &model)?;
        found.remove();
        Ok(())
    })
}

/// 추가가 만드는 모델 항목의 칸들. `context_window`가 없으면 칸을 만들지 않는다.
fn model_fields(name: &str, id: &str, context_window: Option<u64>) -> Vec<(String, CstInputValue)> {
    let mut fields = vec![
        (NAME_KEY.to_owned(), CstInputValue::from(name.trim())),
        (ID_KEY.to_owned(), CstInputValue::from(id.trim())),
    ];
    if let Some(window) = context_window {
        fields.push((CONTEXT_WINDOW_KEY.to_owned(), CstInputValue::from(window)));
    }
    fields
}

/// 수정이 바꿀 칸들. `None`인 칸은 목록에 넣지 않아 그대로 둔다.
fn change_fields(changes: &ModelChanges) -> Vec<(String, CstInputValue)> {
    let mut fields = Vec::new();
    if let Some(id) = &changes.id {
        fields.push((ID_KEY.to_owned(), CstInputValue::from(id.trim())));
    }
    if let Some(window) = changes.context_window {
        fields.push((CONTEXT_WINDOW_KEY.to_owned(), CstInputValue::from(window)));
    }
    if let Some(reasoning) = &changes.reasoning {
        fields.push((
            REASONING_KEY.to_owned(),
            CstInputValue::from(reasoning.trim()),
        ));
    }
    fields
}
