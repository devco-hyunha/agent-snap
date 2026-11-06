//! 두 파일을 함께 바꾸는 저장 진입점: 검증 → 자격 교체 → 설정 교체 순서로 처리합니다.
//!
//! 순서는 D41의 정본이고, 교체 순서 자체는 D40 규칙 3의 정본입니다. 두 파일 저장이 자기
//! 파일을 쓰기 전에 검증을 마치므로, 검증을 통과하지 못한 입력은 자격 파일을 쓰기 전에
//! 거부되어 어느 파일도 바뀌지 않습니다. 검증은 설정 파일 결과를 메모리에 먼저 만드는 단계
//! (`prepared_rename_text`·`prepared_remove_text`)에서 끝나며, 그 뒤에 자격 파일을 교체하고
//! 마지막으로 설정 파일을 교체합니다. 설정 파일 교체에서 실패하면 자격 파일은 이미 바뀐
//! 상태이고, 오류가 어느 쪽이 바뀐 상태인지를 밝힙니다.
//!
//! 이름 바꾸기·삭제처럼 두 파일을 함께 고치는 저장이 이 파일의 책임입니다. 한 파일만 고치는
//! 저장은 `save_config`·`save_credentials`에 이미 있습니다.

use std::path::Path;

use super::super::error::{ConfigError, SaveError};
use super::atomic::{Replace, replace_file};
use super::key_rename::{KeyRename, remove_key_with, rename_key_with};
use super::rename::{prepared_remove_text, prepared_rename_text, write_checked_text};

/// 엔드포인트 이름을 바꿉니다. 순서는 D41대로 검증 → 자격 교체 → 설정 교체입니다(AC 2·AC 3).
///
/// 설정 파일 쪽 검증(02-2 결과 검증, 이름 중복 포함)은 자격 파일을 쓰기 전에 마칩니다.
/// 자격 파일 쪽 검증(02-1 해석)도 자격 파일을 쓰기 전에 막히므로, 어느 검증 거부도 파일을
/// 건드리지 않습니다. 검증이 끝난 설정 파일 텍스트는 자격 교체를 실행하기 전에 메모리에
/// 있습니다.
///
/// 자격 파일에 그 키가 없으면 자격 파일 쪽은 값 없이 성공하고 설정 파일 이름만 바뀝니다.
///
/// # 오류
/// 자격 파일 쪽 검증 거부는 02-1 해석·이름 규칙 오류와 같고, 설정 파일 쪽 검증 거부는 02-2
/// 검증 오류와 같으며 둘 다 파일을 건드리지 않습니다. 교체 실패는 자격 파일이 이미 바뀐
/// 상태라는 것을 밝히는 [`SaveError::EndpointRenameAfterKeySide`]입니다. 입력 주소와 키 값은
/// 오류에 담지 않습니다.
///
/// [`SaveError::EndpointRenameAfterKeySide`]: super::super::error::SaveError::EndpointRenameAfterKeySide
pub fn rename_endpoint(folder: &Path, rename: &KeyRename) -> Result<(), ConfigError> {
    rename_endpoint_with(folder, rename, &replace_file)
}

/// 위 공개 함수가 쓰는 내부 경로입니다. 교체 지점(`replace`)만 시험용 함수로 바꿀 수 있게
/// 분리했습니다. 두 파일 저장이 자기 파일을 원자적으로 교체하므로, 시험은 `replace`로 자기가
/// 쓰는 파일 이름의 교체를 막으면 됩니다.
pub(crate) fn rename_endpoint_with(
    folder: &Path,
    rename: &KeyRename,
    replace: &Replace,
) -> Result<(), ConfigError> {
    // D41의 앞쪽입니다. 설정 파일 편집 결과를 메모리에 만들고 02-2 검증을 여기서 마칩니다.
    // 거부는 자격 교체가 시작하기 전에 끝나므로 파일을 건드리지 않습니다.
    let config_text = prepared_rename_text(folder, &rename.from, &rename.to)?;
    // 검증이 끝났으니 자격 파일 교체(D40 규칙 3의 앞쪽)를 실행합니다. 자격 파일 쪽 검증도
    // 이 단계의 쓰기 전에 막힙니다.
    rename_key_with(folder, rename, replace)?;
    let report_state = |source: std::io::Error| {
        SaveError::EndpointRenameAfterKeySide {
            source: Some(source),
        }
        .into()
    };
    report_config_side(
        write_checked_text(folder, config_text, replace),
        report_state,
    )
}

/// 엔드포인트를 삭제합니다. 순서는 D41대로 검증 → 자격 교체 → 설정 교체입니다.
///
/// 설정 파일 쪽 이름 검사는 자격 파일을 쓰기 전에 마칩니다. 자격 파일에 그 키가 없으면 자격
/// 파일 쪽은 값 없이 성공하고 설정 파일 항목만 지워집니다.
///
/// # 오류
/// 자격 파일 쪽 검증 거부는 02-1 해석·이름 규칙 오류와 같고, 교체 실패는 자격 파일이 이미
/// 바뀐 상태라는 것을 밝히는 [`SaveError::EndpointRemoveAfterKeySide`]입니다. 입력 주소와
/// 키 값은 오류에 담지 않습니다.
///
/// [`SaveError::EndpointRemoveAfterKeySide`]: super::super::error::SaveError::EndpointRemoveAfterKeySide
pub fn remove_endpoint(folder: &Path, endpoint: &str) -> Result<(), ConfigError> {
    remove_endpoint_with(folder, endpoint, &replace_file)
}

/// 위 공개 함수가 쓰는 내부 경로입니다. 교체 지점(`replace`)만 시험용 함수로 바꿀 수 있게
/// 분리했습니다.
pub(crate) fn remove_endpoint_with(
    folder: &Path,
    endpoint: &str,
    replace: &Replace,
) -> Result<(), ConfigError> {
    // D41의 앞쪽입니다. 설정 파일 편집 결과를 메모리에 만들고 이름 검증을 여기서 마칩니다.
    // 바꿀 엔드포인트가 없으면 이 단계에서 이름만 밝히고 거부합니다(spec 표 3번째 줄).
    let config_text = prepared_remove_text(folder, endpoint)?;
    // 검증이 끝났으니 자격 파일의 그 키를 지우는 교체(D40 규칙 3의 앞쪽)를 실행합니다.
    remove_key_with(folder, endpoint, replace)?;
    let report_state = |source: std::io::Error| {
        SaveError::EndpointRemoveAfterKeySide {
            source: Some(source),
        }
        .into()
    };
    report_config_side(
        write_checked_text(folder, Some(config_text), replace),
        report_state,
    )
}

/// 순서의 뒤쪽인 설정 파일 교체 결과를 바꿉니다. 교체 실패만 잡으며, 그 실패가 곧 자격
/// 파일이 이미 바뀐 상태임을 밝히는 순서 규칙의 신호입니다(D40 규칙 3).
///
/// 교체 실패 표시(`SaveIo`)는 대상 파일 종류와 오류 종류만 밝히는 자리라, 원인은
/// 어긋난 상태 오류가 가릴 수 있는 자리인 `io::Error`로 채워 넘깁니다.
fn report_config_side(
    result: Result<(), ConfigError>,
    report_state: impl FnOnce(std::io::Error) -> ConfigError,
) -> Result<(), ConfigError> {
    result.map_err(|error| match error {
        ConfigError::Save(SaveError::SaveIo { .. }) => {
            report_state(std::io::Error::other("교체 실패"))
        }
        other => other,
    })
}
