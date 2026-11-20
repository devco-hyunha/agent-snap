//! 설정 파일(`agent-snap.jsonc`)에 엔드포인트와 모델 옵션을 추가·수정하는 모듈

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

/// 설정 파일에 엔드포인트를 추가하고, 같은 이름이 있으면 그 항목을 고침
///
/// 검증에 걸리면 파일을 건드리지 않고, CST 편집이라 기존 주석은 원문대로 남음
///
/// # 오류
/// - [`super::super::error::SaveError::SaveIo`]: 읽기·쓰기 실패.
/// - [`EndpointSave::validate`]가 밝히는 검증 오류: 규칙 위반.
/// - [`ReadingError::RootNotObject`]: 읽을 수 없는 파일.
pub fn save_endpoint(folder: &Path, save: &EndpointSave) -> Result<(), ConfigError> {
    save_endpoint_with(folder, save, &replace_file)
}

/// 공개 함수가 쓰는 내부 경로. 교체 지점(`replace`)만 시험용으로 바꿀 수 있게 분리
pub(crate) fn save_endpoint_with(
    folder: &Path,
    save: &EndpointSave,
    replace: &Replace,
) -> Result<(), ConfigError> {
    // 거부는 파일 읽기 전에 끝남
    let endpoint = save.check()?;
    let root = read_root(folder)?;
    let config = root
        .object_value_or_create()
        .ok_or(ReadingError::RootNotObject)?;
    merge_endpoint(&config.array_value_or_set("endpoints"), &endpoint)?;
    write_root(folder, &root, replace)
}

/// `endpoints` 배열에 검증한 엔드포인트를 합침. 같은 이름이 있으면 그 항목을 고침
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

/// `models` 배열에 검증한 모델 목록을 합침. 같은 이름 모델은 고치고 없으면 더함
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
        // 선언 값도 주지 않은 항목은 쓰지 않는 규칙. `context_window`와 같은 자리
        if let Some(form) = save.tool_call_form {
            set_string(&object, "tool_call_form", form.to_name());
        }
        // 주지 않은 항목은 칸을 만들지 않는 규칙을 이 한 곳으로 모음
        set_optional(&object, "sampling", sampling_value(save.sampling));
        set_optional(&object, "extra", extra_value(&save.extra));
    }
    Ok(())
}

/// 설정 파일 텍스트를 읽어 CST(원문 트리)로 변환. 파일이 없으면 빈 텍스트로 시작
///
/// 읽기 실패는 대상 파일 종류와 오류 종류만 밝히는 저장 오류로 넘김. 경로를 밝히는 원인은 미포함
pub(crate) fn read_root(folder: &Path) -> Result<CstRootNode, ConfigError> {
    let text = read_text(folder, CONFIG_FILE_NAME)?;
    // 설정 읽기가 정한 JSONC 문법 오류 매핑(`JsoncSyntax`)을 재사용해 줄 번호만 밝힘
    CstRootNode::parse(&text, &ParseOptions::default()).map_err(|error| {
        ReadingError::JsoncSyntax {
            line: error.line_display(),
        }
        .into()
    })
}

/// CST를 텍스트로 되돌려 설정 파일에 씀
fn write_root(folder: &Path, root: &CstRootNode, replace: &Replace) -> Result<(), ConfigError> {
    write_replacing(folder, CONFIG_FILE_NAME, &root.to_string(), replace)
}
