//! 설정 파일(`agent-snap.jsonc`)에 엔드포인트와 모델 옵션을 추가·수정하는 모듈입니다.
//!
//! CST(원문 구조를 주석·공백까지 보관하는 트리) 편집을 쓰므로 사용자가 단 주석과 이번 저장이
//! 손대지 않은 항목이 원문 그대로 남습니다(D40). 파일이 없으면 새 파일로 시작합니다.
//!
//! 쓰기 규칙은 D10·D14·D20·D29를 따릅니다: 엔드포인트에는 `name`·`base_url`·`kind`만 쓰고 키
//! 칸은 쓰지 않으며, 모델은 `endpoints[].models` 중첩에 둡니다. 주지 않은 모델 옵션은 파일에
//! 강제로 쓰지 않습니다(읽을 때 기본값이 정해집니다).
//!
//! 검증하는 부분은 `save_check`가, 이름 바꾸기·삭제의 설정 파일 쪽 편집은 `save_rename`이,
//! 파일 반영의 원자적 교체는 `save_atomic`이 각각 맡고, 두 파일 순서를 묶는 저장은
//! `save_store`가 합니다.

use std::path::Path;

use jsonc_parser::ParseOptions;
use jsonc_parser::cst::{CstArray, CstRootNode};

use super::super::error::{ConfigError, ReadingError};
use super::super::load::CONFIG_FILE_NAME;
use super::atomic::{Replace, replace_file, write_replacing};
use super::check::{CheckedEndpoint, CheckedModel, EndpointSave};
use super::cst::{ensure_object, find_named, set_number, set_optional, set_string};
use super::read::read_text;
use super::values::{extra_value, sampling_value};

/// 설정 파일(`agent-snap.jsonc`)에 엔드포인트를 추가하고, 같은 이름이 있으면 그 항목을 고칩니다.
///
/// 주석과 이번 저장이 손대지 않은 항목은 원문 그대로 남고, 파일이 없으면 새로 만듭니다.
/// 쓰기 전에 02-2 검증 규칙을 거치므로, 규칙에 맞지 않으면 파일을 건드리지 않습니다.
///
/// # 오류
/// 파일 읽기·쓰기 실패는 대상 파일 종류(설정)와 오류 종류만 밝히는 [`super::super::error::SaveError::SaveIo`],
/// 규칙 위반은 [`EndpointSave::validate`]가 밝히는 02-2와 같은 오류, 읽을 수 없는 파일은
/// [`ReadingError::RootNotObject`]입니다. 어느 쪽이든 파일을 건드리기 전에 막습니다.
pub fn save_endpoint(folder: &Path, save: &EndpointSave) -> Result<(), ConfigError> {
    save_endpoint_with(folder, save, &replace_file)
}

/// 위 공개 함수가 쓰는 내부 경로입니다. 교체 지점(`replace`)만 시험용 함수로 바꿀 수 있게
/// 분리했습니다. 교체 실패 시험은 이 인자만 넘기면 됩니다.
pub(crate) fn save_endpoint_with(
    folder: &Path,
    save: &EndpointSave,
    replace: &Replace,
) -> Result<(), ConfigError> {
    // 검증을 먼저 통과시켜 거부 입력은 파일 읽기조차 거치지 않게 합니다.
    let endpoint = save.check()?;
    let root = read_root(folder)?;
    let config = root
        .object_value_or_create()
        .ok_or(ReadingError::RootNotObject)?;
    merge_endpoint(&config.array_value_or_set("endpoints"), &endpoint)?;
    write_root(folder, &root, replace)
}

/// `endpoints` 배열에 검증한 엔드포인트를 합칩니다. 같은 이름이 있으면 그 항목을 고칩니다.
fn merge_endpoint(endpoints: &CstArray, checked: &CheckedEndpoint<'_>) -> Result<(), ConfigError> {
    let object = ensure_object(endpoints, find_named(&endpoints.elements(), &checked.name))
        .ok_or_else(|| ConfigError::invalid_field("endpoints"))?;
    let save = checked.save;
    set_string(&object, "name", &checked.name);
    set_string(&object, "base_url", &save.base_url);
    set_string(&object, "kind", save.kind.to_name());
    merge_models(
        &object.array_value_or_set("models"),
        &checked.models,
        &checked.name,
    )
}

/// `models` 배열에 검증한 모델 목록을 합칩니다. 같은 이름 모델은 고치고 없으면 더합니다.
fn merge_models(
    models: &CstArray,
    checked: &[CheckedModel<'_>],
    endpoint: &str,
) -> Result<(), ConfigError> {
    let invalid_models = || ConfigError::invalid_endpoint_field(endpoint, "models");
    for model in checked {
        let object = ensure_object(models, find_named(&models.elements(), &model.name))
            .ok_or_else(invalid_models)?;
        let save = model.save;
        set_string(&object, "name", &model.name);
        set_string(&object, "id", &model.id);
        if let Some(reasoning) = save.reasoning {
            set_string(&object, "reasoning", reasoning.to_name());
        }
        if let Some(window) = save.context_window {
            set_number(&object, "context_window", &window.to_string());
        }
        // 주지 않은 항목은 칸을 만들지 않는 규칙을 이 한 곳으로 모읍니다(D14·D29).
        set_optional(&object, "sampling", sampling_value(save.sampling));
        set_optional(&object, "extra", extra_value(&save.extra));
    }
    Ok(())
}

/// 설정 파일 텍스트를 읽어 CST(원문 트리)로 바꿉니다. 파일이 없으면 빈 텍스트로 시작합니다.
///
/// 읽기 실패는 `save_cst`가 정하는 저장 오류(대상 파일 종류와 오류 종류만 밝힘)를 그대로
/// 넘깁니다. 경로를 밝히는 원인은 담지 않습니다(02-8 spec 표 2번째 줄).
pub(crate) fn read_root(folder: &Path) -> Result<CstRootNode, ConfigError> {
    let text = read_text(folder, CONFIG_FILE_NAME)?;
    // 02-2의 JSONC 문법 오류 매핑(`JsoncSyntax`)을 그대로 재사용해 줄 번호를 밝힙니다.
    CstRootNode::parse(&text, &ParseOptions::default()).map_err(|error| {
        ReadingError::JsoncSyntax {
            line: error.line_display(),
        }
        .into()
    })
}

/// CST를 텍스트로 되돌려 설정 파일에 씁니다.
fn write_root(folder: &Path, root: &CstRootNode, replace: &Replace) -> Result<(), ConfigError> {
    write_replacing(folder, CONFIG_FILE_NAME, &root.to_string(), replace)
}
