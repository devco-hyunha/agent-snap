//! 설정 파일 이름 바꾸기·삭제 저장 시험(02-9 작업 3의 설정 쪽 책임).
//!
//! 자격 파일만 다루는 이름 바꾸기·삭제 시험은 `credentials_rename.rs`로 나눕니다. 이 파일은
//! 두 파일을 함께 고치는 저장 진입점과 설정 파일 쪽 동작을 봅니다. 파일 텍스트 견본은 저장을
//! 보는 여러 시험이 함께 쓰므로 지원부(`save_samples.rs`)에서 가져옵니다.

use crate::config::CONFIG_FILE_NAME;
use crate::config::CREDENTIALS_FILE_NAME;
use crate::config::remove_endpoint;
use crate::config::rename_endpoint;
use crate::config::save::store::{remove_endpoint_with, rename_endpoint_with};
use crate::config::{
    ConfigError, EndpointIssue, KeyRename, SaveError, SaveIoKind, SaveIoTarget, ValidationError,
};

use super::super::support::{
    CONFIG_TEXT, CREDENTIALS_TEXT, TWO_ENDPOINTS_TEXT, TempDir, assert_error_hides_secrets,
    blocking_replacement, parse_ok,
};

/// AC 1(설정 파일 쪽 교체 실패): 설정 파일 교체만 막으면 설정 파일은 그대로이고 임시 파일이
/// 남지 않습니다. D41 순서는 검증 후 자격 교체, 그다음 설정 교체이므로 자격 파일은 이미 바뀐
/// 상태이고, 오류가 그 상태를 밝힙니다. 바뀐 자격 파일 상태와 오류 문구의 상세 확인은
/// `config_only_replacement_failure_reports_key_side_already_changed`가 맡습니다.
#[test]
fn config_side_replacement_failure_keeps_config_file_and_leaves_no_temp_file() {
    let folder = TempDir::new("설정-교체-실패");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    let error = rename_endpoint_with(
        folder.path(),
        &rename_main_to_main2(),
        &blocking_replacement(CONFIG_FILE_NAME),
    )
    .expect_err("설정 파일 교체가 실패해야 합니다");

    assert_eq!(
        folder.read(CONFIG_FILE_NAME),
        CONFIG_TEXT,
        "설정 파일이 그대로여야 합니다"
    );
    assert!(
        folder.read(CREDENTIALS_FILE_NAME).contains("\"main2\""),
        "D41 순서대로 자격 파일 교체까지 끝난 상태여야 합니다"
    );
    folder.assert_no_temp_files();
    assert!(
        matches!(
            error,
            ConfigError::Save(SaveError::EndpointRenameAfterKeySide { .. })
        ),
        "설정 파일 교체 실패이고 자격 파일이 바뀐 상태를 밝혀야 합니다: {error}"
    );
    assert_error_hides_secrets("설정 교체 실패", &error);
}

/// AC 1(자격 파일 쪽 교체 실패): 자격 파일 교체가 실패하면 두 파일이 그대로이고 설정 파일
/// 교체가 일어나지 않습니다(D40 규칙 3의 앞쪽이 먼저 끝나지 않는 상태).
#[test]
fn key_side_replacement_failure_stops_before_config_replacement() {
    let folder = TempDir::new("자격-교체-실패");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    let error = rename_endpoint_with(
        folder.path(),
        &rename_main_to_main2(),
        &blocking_replacement(CREDENTIALS_FILE_NAME),
    )
    .expect_err("자격 파일 교체가 실패해야 합니다");

    assert_eq!(
        folder.read(CREDENTIALS_FILE_NAME),
        CREDENTIALS_TEXT,
        "자격 파일이 그대로여야 합니다"
    );
    assert_eq!(
        folder.read(CONFIG_FILE_NAME),
        CONFIG_TEXT,
        "설정 파일 교체는 순서의 앞쪽이 끝나야 일어나므로 이 순서에서 일어나면 안 됩니다"
    );
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

/// AC 3: 이름 바꾸기 저장이 자격 파일 교체 뒤에 설정 파일 교체를 일으킵니다.
#[test]
fn renaming_endpoint_replaces_credentials_before_config() {
    let folder = TempDir::new("이름-변경-순서");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    rename_endpoint(folder.path(), &rename_main_to_main2())
        .expect("이름 바꾸기 저장이 성공해야 합니다");

    let config_text = folder.read(CONFIG_FILE_NAME);
    assert!(
        config_text.contains(r#""name": "main2""#),
        "설정 파일 이름이 바뀌어야 합니다: {config_text}"
    );
    assert!(
        folder.read(CREDENTIALS_FILE_NAME).contains("\"main2\""),
        "자격 파일 키 이름이 바뀌어야 합니다"
    );
    assert!(
        parse_ok(&config_text).endpoints[0].name == "main2",
        "바뀐 이름으로 읽혀야 합니다"
    );
    folder.assert_no_temp_files();
}

/// AC 3: 설정 파일 교체만 실패하면 자격 파일은 새 이름 키로 바뀐 상태이고, 오류가 어떤 쪽이
/// 바뀐 상태인지를 밝힙니다.
#[test]
fn config_only_replacement_failure_reports_key_side_already_changed() {
    let folder = TempDir::new("설정-교체-실패-상태");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    let error = rename_endpoint_with(
        folder.path(),
        &rename_main_to_main2(),
        &blocking_replacement(CONFIG_FILE_NAME),
    )
    .expect_err("설정 파일 교체만 실패해야 합니다");

    assert!(
        folder.read(CREDENTIALS_FILE_NAME).contains("\"main2\""),
        "자격 파일은 새 이름 키로 바뀐 상태여야 합니다"
    );
    assert!(
        matches!(
            error,
            ConfigError::Save(SaveError::EndpointRenameAfterKeySide { .. })
        ),
        "오류가 자격 파일이 바뀐 상태임을 밝혀야 합니다: {error}"
    );
    assert_error_hides_secrets("이름 바꾸기 어긋난 상태", &error);
    folder.assert_no_temp_files();
}

/// AC 1, 3: 엔드포인트 삭제도 같은 순서를 따르고, 자격 파일 키만 지워진 채 설정 파일 교체가
/// 실패하면 그 상태를 밝힙니다.
#[test]
fn removing_endpoint_replaces_credentials_before_config_and_reports_state() {
    let folder = TempDir::new("삭제-순서");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    let error = remove_endpoint_with(
        folder.path(),
        "main",
        &blocking_replacement(CONFIG_FILE_NAME),
    )
    .expect_err("삭제 저장이 실패해야 합니다");

    let credentials_text = folder.read(CREDENTIALS_FILE_NAME);
    assert!(
        !credentials_text.contains("\"main\""),
        "자격 파일 키가 먼저 지워진 상태여야 합니다: {credentials_text}"
    );
    assert_eq!(
        folder.read(CONFIG_FILE_NAME),
        CONFIG_TEXT,
        "설정 파일은 그대로여야 합니다"
    );
    assert!(
        matches!(
            error,
            ConfigError::Save(SaveError::EndpointRemoveAfterKeySide { .. })
        ),
        "오류가 자격 파일 키가 지워진 상태임을 밝혀야 합니다: {error}"
    );
    folder.assert_no_temp_files();
}

/// AC 2의 설정 파일 쪽 검증: 결과를 02-2 검증으로 다시 읽은 거부(같은 이름 항목이 있는
/// 이름 바꾸기)는 파일을 건드리지 않습니다. 검증이 자격 교체보다 앞에서 끝나므로 자격 파일까지
/// 그대로여야 합니다(D41).
#[test]
fn duplicate_name_rename_rejects_before_touching_files() {
    let folder = TempDir::new("이름-중복-거부");
    folder.write(CONFIG_FILE_NAME, TWO_ENDPOINTS_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    let _error = rename_endpoint(folder.path(), &rename_main_to_main2())
        .expect_err("새 이름 항목이 있으면 거부되어야 합니다");

    assert_eq!(
        folder.read(CONFIG_FILE_NAME),
        TWO_ENDPOINTS_TEXT,
        "설정 파일 검증 거부가 일어나 파일을 건드리면 안 됩니다"
    );
    assert_eq!(
        folder.read(CREDENTIALS_FILE_NAME),
        CREDENTIALS_TEXT,
        "검증이 자격 교체보다 앞에서 끝나 자격 파일도 그대로여야 합니다"
    );
    folder.assert_no_temp_files();
}

/// AC 3의 경계: 자격 파일에 그 키가 없으면 자격 파일 쪽은 값 없이 성공하고 설정 파일만
/// 바뀝니다.
#[test]
fn rename_without_key_succeeds_and_only_changes_config() {
    let folder = TempDir::new("키-없는-이름-변경");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    // 자격 파일에 `main` 키가 없는 상태입니다.
    folder.write(CREDENTIALS_FILE_NAME, r#"{ "api_keys": {} }"#);

    rename_endpoint(folder.path(), &rename_main_to_main2())
        .expect("키가 없는 이름 바꾸기가 성공해야 합니다");

    assert!(
        parse_ok(&folder.read(CONFIG_FILE_NAME)).endpoints[0].name == "main2",
        "설정 파일 이름만 바뀌어야 합니다"
    );
    assert!(
        !folder.read(CREDENTIALS_FILE_NAME).contains("\"main\""),
        "자격 파일에 옛 이름 키가 없으므로 새 이름 키가 생겨야 합니다"
    );
}

/// AC 3의 설정 파일 쪽 이름 바꾸기만 실패하는 상태를 확인합니다. 자격 파일 쪽 이름 바꾸기는
/// 값 없이 성공하고, 설정 파일 교체만 실패합니다.
#[test]
fn config_only_rename_failure_reports_key_side_state_without_key_change() {
    let folder = TempDir::new("설정-이름-변경-교체실패");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, r#"{ "api_keys": {} }"#);

    let error = rename_endpoint_with(
        folder.path(),
        &rename_main_to_main2(),
        &blocking_replacement(CONFIG_FILE_NAME),
    )
    .expect_err("설정 파일 교체만 실패해야 합니다");

    assert_eq!(
        folder.read(CONFIG_FILE_NAME),
        CONFIG_TEXT,
        "설정 파일은 그대로여야 합니다"
    );
    assert!(
        matches!(
            error,
            ConfigError::Save(SaveError::EndpointRenameAfterKeySide { .. })
        ),
        "어긋난 상태를 밝히는 오류여야 합니다: {error}"
    );
}

/// 이름 바꾸기, 삭제가 한 번의 호출로 두 파일 저장을 처리하는 진입점임을 확인합니다(AC 4:
/// 저장 API가 여러 함수를 섞지 않음, dev-principles 3절 DRY).
#[test]
fn entry_points_process_both_files_in_one_call() {
    let rename_folder = TempDir::new("진입점-이름-변경");
    rename_folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    rename_folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);
    rename_endpoint(
        rename_folder.path(),
        &KeyRename {
            from: "main".to_owned(),
            to: "main2".to_owned(),
        },
    )
    .expect("이름 바꾸기 저장은 한 번의 호출이어야 합니다");

    let remove_folder = TempDir::new("진입점-삭제");
    remove_folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    remove_folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);
    remove_endpoint(remove_folder.path(), "main").expect("삭제 저장은 한 번의 호출이어야 합니다");

    assert!(
        parse_ok(&rename_folder.read(CONFIG_FILE_NAME)).endpoints[0].name == "main2"
            && rename_folder
                .read(CREDENTIALS_FILE_NAME)
                .contains("\"main2\""),
        "두 파일이 함께 바뀌어야 합니다"
    );
    assert!(
        parse_ok(&remove_folder.read(CONFIG_FILE_NAME))
            .endpoints
            .is_empty()
            && !remove_folder
                .read(CREDENTIALS_FILE_NAME)
                .contains("\"main\""),
        "두 파일이 함께 지워져야 합니다"
    );
}

/// AC 2: 02-2 이름 규칙에 맞지 않는 이름 바꾸기 요청은 두 파일을 모두 건드리지 않습니다.
#[test]
fn invalid_rename_leaves_both_files_untouched() {
    let folder = TempDir::new("이름-검증-실패");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    folder.write(CREDENTIALS_FILE_NAME, CREDENTIALS_TEXT);

    let error = rename_endpoint(
        folder.path(),
        &KeyRename {
            from: "main".to_owned(),
            to: "a/b".to_owned(),
        },
    )
    .expect_err("새 이름 규칙 위반이 거부되어야 합니다");

    assert_eq!(folder.read(CONFIG_FILE_NAME), CONFIG_TEXT);
    assert_eq!(folder.read(CREDENTIALS_FILE_NAME), CREDENTIALS_TEXT);
    folder.assert_no_temp_files();
    assert!(
        matches!(
            error,
            ConfigError::Validation(ValidationError::Endpoint(
                EndpointIssue::EndpointNameHasSlash { .. }
            ))
        ),
        "02-2와 같은 이름 규칙 위반이어야 합니다: {error}"
    );
}

/// 이름 바꾸기 정보(`main`에서 `main2`로 바꿈)입니다.
fn rename_main_to_main2() -> KeyRename {
    KeyRename {
        from: "main".to_owned(),
        to: "main2".to_owned(),
    }
}
