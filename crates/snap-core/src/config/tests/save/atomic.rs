//! 안전한 쓰기 시험(02-8 작업 4 AC 1~4)의 Windows를 타지 않는 부분.
//!
//! Windows 실측 시험은 `windows_replace.rs`로 따로 두어, Windows가 움직이는 자리에서만
//! 돌아갑니다. 여기에는 검증 거부와 교체 실패 일반 시험만 남기고 Windows 전용 APIs를
//! 쓰지 않습니다. 교체 지점을 막아서 기존 파일이 그대로이고 임시 파일이 남지 않는지,
//! 검증 거부 입력이 파일을 안 바꾸는지 봅니다. 파일 텍스트 견본은 저장을 보는 여러 시험이
//! 함께 쓰는 지원부(`save_samples.rs`)를 씁니다.

use crate::config::save::config::save_endpoint_with;
use crate::config::save::credentials::save_key_with;
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointIssue, EndpointKind,
    EndpointSave, SaveError, SaveIoKind, SaveIoTarget, ValidationError,
};
use crate::secrets::SecretValue;

use super::super::support::{
    CONFIG_TEXT, CREDENTIALS_TEXT, KEY_VALUE, TempDir, blocking_replacement,
};

/// AC 1(키 값 저장 쪽 교체 실패): 키 값 저장이 교체에 실패하면 자격 파일은 그대로이고
/// 임시 파일이 남지 않습니다.
#[test]
fn key_save_replacement_failure_keeps_file_and_leaves_no_temp_file() {
    let folder = TempDir::new("키-교체-실패");
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    let error = save_key_with(
        folder.path(),
        "new_ep",
        &SecretValue::new(KEY_VALUE),
        &blocking_replacement(CREDENTIALS_FILE_NAME),
    )
    .expect_err("키 저장 교체가 실패해야 합니다");

    assert_eq!(folder.read(CREDENTIALS_FILE_NAME), CREDENTIALS_TEXT);
    folder.assert_no_temp_files();
    assert!(
        matches!(
            error,
            ConfigError::Save(SaveError::SaveIo {
                target: SaveIoTarget::Credentials,
                kind: SaveIoKind::Replace
            })
        ),
        "자격 파일 쪽 교체 실패여야 합니다: {error}"
    );
}

/// AC 2: 02-2 검증을 통과하지 못하는 설정 저장 입력은 두 파일을 모두 건드리지 않습니다.
#[test]
fn validation_failure_leaves_both_files_untouched() {
    let folder = TempDir::new("검증-실패");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);
    let save = EndpointSave {
        name: "main2".to_owned(),
        base_url: "ftp://leak.invalid/v1".to_owned(),
        kind: EndpointKind::OpenAi,
        models: Vec::new(),
    };

    let error = save_endpoint_with(
        folder.path(),
        &save,
        &blocking_replacement(CONFIG_FILE_NAME),
    )
    .expect_err("주소 검증이 거부되어야 합니다");

    assert_eq!(folder.read(CONFIG_FILE_NAME), CONFIG_TEXT);
    assert_eq!(folder.read(CREDENTIALS_FILE_NAME), CREDENTIALS_TEXT);
    folder.assert_no_temp_files();
    assert!(
        matches!(
            error,
            ConfigError::Validation(ValidationError::Endpoint(
                EndpointIssue::InvalidBaseUrl { .. }
            ))
        ),
        "02-2와 같은 오류여야 합니다: {error}"
    );
}
