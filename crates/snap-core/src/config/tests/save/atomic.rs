//! 안전한 쓰기 단위 테스트(02-8 작업 4 AC 1~4).
//!
//! 이름 바꾸기·삭제 저장을 보는 테스트는 `config/tests/save/rename.rs`로 옮겼고, 여기에는
//! 교체 지점 시험과 검증 거부가 남습니다. `replace`로 자기가 쓰는 파일 이름의 교체를 막고
//! 기존 파일이 그대로이고 임시 파일이 남지 않는지, 검증 실패 입력이 파일을 안 바꾸는지 봅니다.

use crate::config::save::atomic::replace_file;
use crate::config::save::config::save_endpoint_with;
use crate::config::save::credentials::save_key_with;
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointKind, EndpointSave, SaveError,
    SaveIoKind, SaveIoTarget, ValidationError,
};
use crate::secrets::SecretValue;

use super::super::support::{KEY_VALUE, TempDir, blocking_replacement, endpoint_save};

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
            ConfigError::Validation(ValidationError::InvalidBaseUrl { .. })
        ),
        "02-2와 같은 오류여야 합니다: {error}"
    );
}

/// AC 1(Windows 실측): 대상 파일을 다른 열림 상태로 열어 두면 표준 교체 지점 `std::fs::rename`이
/// 실제로 실패하는지 확인합니다. 기존 파일은 그대로이고 임시 파일이 남지 않음을 봅니다.
///
/// Windows가 아닌 곳에서는 이 시험을 건너뜁니다. 건너뛰는 이유: Windows 파일 공유 규칙
/// (열려 있는 파일을 `std::fs::rename`으로 바꿀 때 생기는 공유 위반)을 보는 측정이라
/// Windows가 아닌 운영체제에서는 같은 실패가 일어나지 않기 때문입니다.
#[cfg(windows)]
#[test]
fn windows_open_target_file_fails_standard_rename_and_keeps_file() {
    use std::os::windows::fs::OpenOptionsExt;

    let folder = TempDir::new("windows-열린-파일-교체-실패");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    // 대상을 공유 설정 없이(share_mode 0) 열어 둡니다. 이 상태가 표준 교체의 대상 열림
    // (DELETE 요청)을 공유 위반으로 막는 Windows 조건입니다.
    let target_open = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(folder.path().join(CONFIG_FILE_NAME))
        .expect("대상 파일을 여는 것은 성공해야 합니다");
    let error = save_endpoint_with(folder.path(), &main_endpoint_save(), &replace_file)
        .expect_err("대상 파일이 공유 없이 열려 있으므로 표준 교체가 실측 실패해야 합니다");
    assert!(
        matches!(
            error,
            ConfigError::Save(SaveError::SaveIo {
                target: SaveIoTarget::Config,
                kind: SaveIoKind::SharingViolation
            })
        ),
        "설정 파일 쪽 교체 실패여야 합니다: {error}"
    );
    // 대상을 연 열림 상태로만 읽을 수 있습니다. 그 열림으로 읽은 내용이 원문 그대로라면
    // 기존 파일이 보존된 것입니다.
    use std::io::Read;
    let mut opened = target_open;
    let mut kept = String::new();
    opened
        .read_to_string(&mut kept)
        .expect("열려 있는 상태로는 읽을 수 있어야 합니다");
    assert_eq!(kept, CONFIG_TEXT, "기존 파일이 그대로여야 합니다");
    folder.assert_no_temp_files();
}

/// AC 1(Windows 실측 2): 읽기 전용 열림(`std::fs::File::open`)은 대상 파일과 읽기를 공유하므로
/// 표준 교체가 성공합니다. 읽기 전용 열림만으로는 교체 실패가 일어나지 않는다는 실측
/// 결과를 남깁니다(D40의 확인하지 못한 것을 확인한 결과입니다).
#[cfg(windows)]
#[test]
fn windows_read_only_open_allows_standard_rename() {
    let folder = TempDir::new("windows-읽기-열림-교체-성공");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    let target_open = std::fs::File::open(folder.path().join(CONFIG_FILE_NAME))
        .expect("대상 파일을 여는 것은 성공해야 합니다");
    let result = save_endpoint_with(folder.path(), &main_endpoint_save(), &replace_file);
    drop(target_open);
    result.expect("읽기 전용 열림에서는 표준 교체가 성공해야 합니다");
    assert!(
        folder
            .read(CONFIG_FILE_NAME)
            .contains("http://local.invalid/v1"),
        "교체가 반영되어 있어야 합니다"
    );
    folder.assert_no_temp_files();
}

/// 검증만 통과하면 그대로 저장할 수 있는 `main` 엔드포인트 정보입니다.
fn main_endpoint_save() -> EndpointSave {
    endpoint_save("main", LOCAL_BASE_URL)
}

/// 교체 성공 확인에 쓰는 로컬 엔드포인트 주소입니다.
const LOCAL_BASE_URL: &str = "http://local.invalid/v1";

/// `main` 엔드포인트 하나가 있고 사용자 주석이 있는 설정 파일 텍스트입니다.
const CONFIG_TEXT: &str = r#"{
  // 사용자 주석입니다.
  "endpoints": [{"name": "main", "base_url": "http://local.invalid/v1", "kind": "ollama"}]
}"#;
/// 자격 파일 키 `main`이 있고 사용자 주석이 있는 자격 파일 텍스트입니다.
const CREDENTIALS_TEXT: &str = r#"{
  // 사용자 주석입니다.
  "api_keys": { "main": "main-key-111" }
}"#;
