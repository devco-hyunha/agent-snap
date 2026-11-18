//! 두 파일을 함께 바꾸는 저장 진입점: 검증 → 자격 교체 → 설정 교체 순서로 처리

use std::path::Path;

use super::super::error::{ConfigError, SaveError};
use super::atomic::{Replace, replace_file};
use super::key_rename::{KeyRename, remove_key_with, rename_key_with};
use super::rename::{prepared_remove_text, prepared_rename_text, write_checked_text};

/// 엔드포인트 이름을 바꿈. 검증 → 자격 교체 → 설정 교체 순서
///
/// 모든 검증은 자격 파일을 쓰기 전에 마침. 검증에 걸리면 파일을 건드리지 않고, 검증이 끝난 설정 파일 텍스트는 자격 교체 전에 메모리에 있음. 자격 파일에 그 키가 없으면 자격 쪽은 값 없이 성공하고 설정 파일 이름만 바뀜
///
/// # 오류
/// 검증 거부는 해석·이름 규칙 오류와 검증 오류와 같음. 교체 실패는 자격 파일이 이미 바뀐 상태임을 밝히는 [`super::super::error::SaveError::EndpointRenameAfterKeySide`]. 입력 주소와 키 값은 미포함
pub fn rename_endpoint(folder: &Path, rename: &KeyRename) -> Result<(), ConfigError> {
    rename_endpoint_with(folder, rename, &replace_file)
}

/// 공개 함수가 쓰는 내부 경로. 교체 지점(`replace`)만 시험용으로 바꿀 수 있게 분리
pub(crate) fn rename_endpoint_with(
    folder: &Path,
    rename: &KeyRename,
    replace: &Replace,
) -> Result<(), ConfigError> {
    // 순서 앞쪽. 설정 파일 편집 결과를 메모리에 만들고 검증을 여기서 마침
    // 거부는 자격 교체가 시작하기 전에 끝나 파일을 건드리지 않음
    let config_text = prepared_rename_text(folder, &rename.from, &rename.to)?;
    // 검증 종료. 자격 파일 교체를 실행. 자격 파일 쪽 검증도 이 단계의 쓰기 전에 막힘
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

/// 엔드포인트를 삭제함. 검증 → 자격 교체 → 설정 교체 순서
///
/// 설정 파일 쪽 이름 검사는 자격 파일을 쓰기 전에 마침. 자격 파일에 그 키가 없으면 자격 쪽은 값 없이 성공하고 설정 파일 항목만 지워짐
///
/// # 오류
/// 자격 파일 쪽 검증 거부는 해석·이름 규칙 오류와 같음. 교체 실패는 자격 파일이 이미 바뀐 상태임을 밝히는 [`super::super::error::SaveError::EndpointRemoveAfterKeySide`]. 입력 주소와 키 값은 미포함
pub fn remove_endpoint(folder: &Path, endpoint: &str) -> Result<(), ConfigError> {
    remove_endpoint_with(folder, endpoint, &replace_file)
}

/// 위 공개 함수가 쓰는 내부 경로. 교체 지점(`replace`)만 시험용으로 바꿀 수 있게 분리
pub(crate) fn remove_endpoint_with(
    folder: &Path,
    endpoint: &str,
    replace: &Replace,
) -> Result<(), ConfigError> {
    // 순서 앞쪽. 설정 파일 편집 결과를 메모리에 만들고 이름 검증을 여기서 마침
    // 바꿀 엔드포인트가 없으면 이 단계에서 이름만 밝히고 거부
    let config_text = prepared_remove_text(folder, endpoint)?;
    // 검증 종료. 자격 파일의 그 키를 지우는 교체를 실행
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

/// 순서 뒤쪽인 설정 파일 교체 결과를 바꿈. 교체 실패만 잡음. 그 실패는 자격 파일이 이미 바뀐 상태임을 밝히는 순서 신호
///
/// 교체 실패 표시(`SaveIo`)는 대상 파일 종류와 오류 종류만 밝히는 자리라, 원인은 어긋난 상태 오류가 가릴 수 있는 `io::Error`로 채워 넘김
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
