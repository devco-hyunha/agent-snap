//! 이름 바꾸기, 삭제 저장을 검증하는 테스트(02-9 작업 3: 책임을 파일로 옮김).
//!
//! 이동 전 `save_atomic.rs`와 `save_credentials.rs`에 있던 본문 그대로 옮겼습니다.
//! 이름 바꾸기와 삭제는 두 파일을 한 번의 저장으로 처리합니다. 검증 거부가 두 파일을
//! 모두 건드리지 않는지, 저장 순서가 D41(검증 후 자격 교체, 그다음 설정 교체)을 따르는지,
//! 어긋난 상태를 오류가 밝히는지, 자격 파일 쪽 이름 바꾸기와 삭제 동작을 봅니다.

use crate::config::save::key_rename::{remove_key, rename_key};
use crate::config::save::store::{remove_endpoint_with, rename_endpoint_with};
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, KeyRename, SaveError, SaveIoKind,
    SaveIoTarget, ValidationError, remove_endpoint, rename_endpoint,
};

use super::super::support::{
    TempDir, assert_error_hides_secrets, blocking_replacement, count_key_entries, key_value,
    parse_credentials, parse_ok,
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
            ConfigError::Validation(ValidationError::EndpointNameHasSlash { .. })
        ),
        "02-2와 같은 이름 규칙 위반이어야 합니다: {error}"
    );
}

/// AC 3: 자격 파일의 이름을 바꾸면 키 이름이 새 이름으로 바뀌고 값은 그대로입니다.
#[test]
fn renaming_key_moves_the_key_name_and_keeps_the_value() {
    let folder = TempDir::new("자격-이름-변경");
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_FILE);

    rename_key(
        folder.path(),
        &KeyRename {
            from: " main ".to_owned(),
            to: "메인-서버".to_owned(),
        },
    )
    .expect("이름 바꾸기가 성공해야 합니다");

    let text = folder.read(CREDENTIALS_FILE_NAME);
    let credentials = parse_credentials(&text);
    assert_eq!(
        key_value(&credentials, "메인-서버").as_deref(),
        Some(MAIN_KEY_VALUE),
        "값은 그대로여야 합니다"
    );
    assert_eq!(
        key_value(&credentials, "main"),
        None,
        "옛 이름 키는 남아 있으면 안 됩니다: {text}"
    );
    assert!(
        text.contains(OTHER_KEY_TEXT) && text.contains(ROOT_EXTRA_TEXT),
        "주석과 다른 키가 그대로여야 합니다: {text}"
    );
}

/// AC 3: 새 이름 키가 이미 있으면 값을 옮기고 이름이 겹치는 칸을 남기지 않습니다.
#[test]
fn renaming_key_onto_existing_target_key_keeps_one_entry_with_moved_value() {
    let folder = TempDir::new("자격-이름-변경-선두");
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_FILE);

    // `stage_ep` 칸이 이미 있는 자리로 `main` 키를 옮깁니다.
    rename_key(
        folder.path(),
        &KeyRename {
            from: "main".to_owned(),
            to: "stage_ep".to_owned(),
        },
    )
    .expect("새 이름 키가 있는 이름 바꾸기도 성공해야 합니다");

    let text = folder.read(CREDENTIALS_FILE_NAME);
    let credentials = parse_credentials(&text);
    assert_eq!(
        key_value(&credentials, "stage_ep").as_deref(),
        Some(MAIN_KEY_VALUE),
        "옮긴 값이 읽혀야 합니다"
    );
    assert_eq!(key_value(&credentials, "main"), None);
    assert_eq!(
        count_key_entries(&text, "stage_ep"),
        1,
        "같은 이름 키 칸이 둘 이상이면 안 됩니다: {text}"
    );
}

/// AC 3의 경계: 이름이 겹치는 칸이 둘인 파일에서 이름을 바꾸면 칸 하나로 정리됩니다.
#[test]
fn renaming_key_over_duplicate_names_collapses_to_one_entry() {
    let folder = TempDir::new("자격-이름-변경-중복");
    folder.write(CREDENTIALS_FILE_NAME, DUPLICATE_KEY_FILE);

    rename_key(
        folder.path(),
        &KeyRename {
            from: "main".to_owned(),
            to: "main-새-이름".to_owned(),
        },
    )
    .expect("이름 중복이 있는 파일의 이름 바꾸기도 성공해야 합니다");

    let text = folder.read(CREDENTIALS_FILE_NAME);
    let credentials = parse_credentials(&text);
    // 02-1 해석 규칙(같은 이름은 마지막 칸)과 같은 마지막 값이 왕복으로 읽혀야 합니다.
    assert_eq!(
        key_value(&credentials, "main-새-이름").as_deref(),
        Some("last-value"),
        "마지막 칸의 값이 그대로 읽혀야 합니다"
    );
    assert_eq!(key_value(&credentials, "main"), None);
}

/// AC 3의 경계: 자격 파일에 그 키가 없으면 이름을 바꾸지 않고 파일을 건드리지 않습니다.
#[test]
fn renaming_missing_key_leaves_file_untouched() {
    let folder = TempDir::new("자격-이름-변경-없음");
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_FILE);

    rename_key(
        folder.path(),
        &KeyRename {
            from: "없는-서버".to_owned(),
            to: "메인-서버".to_owned(),
        },
    )
    .expect("키가 없는 이름 바꾸기는 성공입니다");

    assert_eq!(
        folder.read(CREDENTIALS_FILE_NAME),
        ANNOTATED_FILE,
        "파일이 그대로여야 합니다"
    );
}

/// AC 3의 경계: trim한 이름이 같으면 파일을 건드리지 않습니다.
#[test]
fn renaming_same_name_leaves_file_untouched() {
    let folder = TempDir::new("자격-이름-변경-동일");
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_FILE);

    rename_key(
        folder.path(),
        &KeyRename {
            from: " main ".to_owned(),
            to: "main ".to_owned(),
        },
    )
    .expect("같은 이름 바꾸기는 성공입니다");

    assert_eq!(folder.read(CREDENTIALS_FILE_NAME), ANNOTATED_FILE);
}

/// AC 4: 엔드포인트를 삭제하면 자격 파일의 그 키가 지워집니다.
#[test]
fn removing_key_deletes_the_endpoint_key() {
    let folder = TempDir::new("자격-키-삭제");
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_FILE);

    remove_key(folder.path(), " main ").expect("키 삭제가 성공해야 합니다");

    let text = folder.read(CREDENTIALS_FILE_NAME);
    let credentials = parse_credentials(&text);
    assert_eq!(
        key_value(&credentials, "main"),
        None,
        "키가 지워져야 합니다: {text}"
    );
    assert_eq!(
        key_value(&credentials, "stage_ep").as_deref(),
        Some(OTHER_KEY_VALUE),
        "다른 키가 그대로여야 합니다"
    );
    assert!(
        text.contains(ROOT_EXTRA_TEXT) && text.contains("// 사용자 주석입니다."),
        "주석과 다른 항목이 그대로여야 합니다: {text}"
    );
}

/// AC 4: 그 키가 없으면 자격 파일은 그대로입니다.
#[test]
fn removing_missing_key_leaves_file_untouched() {
    let folder = TempDir::new("자격-키-삭제-없음");
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_FILE);

    remove_key(folder.path(), "없는-서버").expect("키가 없는 삭제는 성공입니다");

    assert_eq!(
        folder.read(CREDENTIALS_FILE_NAME),
        ANNOTATED_FILE,
        "파일이 그대로여야 합니다"
    );
}

/// AC 4: 이름이 겹치는 칸이 둘인 파일에서 삭제하면 그 이름 칸을 함께 지웁니다.
#[test]
fn removing_key_over_duplicate_names_removes_all_entries() {
    let folder = TempDir::new("자격-키-삭제-중복");
    folder.write(CREDENTIALS_FILE_NAME, DUPLICATE_KEY_FILE);

    remove_key(folder.path(), "main").expect("중복 키 삭제가 성공해야 합니다");

    let text = folder.read(CREDENTIALS_FILE_NAME);
    let credentials = parse_credentials(&text);
    assert_eq!(key_value(&credentials, "main"), None);
    assert_eq!(
        key_value(&credentials, "stage_ep").as_deref(),
        Some(OTHER_KEY_VALUE),
        "다른 키가 그대로여야 합니다"
    );
}

/// 02-2 이름 규칙에 맞지 않는 이름 바꾸기·삭제는 파일을 건드리지 않고 거부합니다.
/// (이동 전 `invalid_endpoint_name_rejects_before_touching_file`의 이름 바꾸기·삭제 부분)
#[test]
fn invalid_endpoint_name_rejects_rename_and_remove_before_touching_file() {
    for (label, endpoint) in [("빈 이름", "   "), ("슬래시 이름", "a/b")] {
        let folder = TempDir::new("자격-거부");
        folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_FILE);

        let rename = KeyRename {
            from: endpoint.to_owned(),
            to: "main".to_owned(),
        };
        let error = rename_key(folder.path(), &rename)
            .expect_err(&format!("{label} 이름 바꾸기는 거부되어야 합니다"));
        assert_eq!(
            folder.read(CREDENTIALS_FILE_NAME),
            ANNOTATED_FILE,
            "{label}: 이름 바꾸기가 파일을 바꾸었습니다"
        );
        assert_error_hides_secrets(label, &error);
        let error = remove_key(folder.path(), endpoint)
            .expect_err(&format!("{label} 키 삭제가 거부되어야 합니다"));
        assert_eq!(
            folder.read(CREDENTIALS_FILE_NAME),
            ANNOTATED_FILE,
            "{label}: 키 삭제가 파일을 바꾸었습니다"
        );
        assert_error_hides_secrets(label, &error);
    }
}

/// 이름 바꾸기 정보(`main`에서 `main2`로 바꿈)입니다.
fn rename_main_to_main2() -> KeyRename {
    KeyRename {
        from: "main".to_owned(),
        to: "main2".to_owned(),
    }
}

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
/// `main2` 항목이 이미 있어 이름 바꾸기가 02-2 검증에서 거부되는 설정 파일 텍스트입니다.
const TWO_ENDPOINTS_TEXT: &str = r#"{
  "endpoints": [
    {"name": "main", "base_url": "http://local.invalid/v1", "kind": "ollama"},
    {"name": "main2", "base_url": "http://local.invalid/v1", "kind": "ollama"}
  ]
}"#;
/// 자격 파일 쪽 이름 바꾸기, 삭제 검사가 쓰는 주석, 다른 항목이 있는 파일입니다.
const ANNOTATED_FILE: &str = r#"// 사용자 주석입니다.
{
  /* api_keys 밖 항목은 무시합니다 */
  "endpoint": [],
  "other_root_key": 1,
  "api_keys": {
    // 메인 서버
    "main": "main-key-111",
    // 다른 서버 키
    "stage_ep": "stage-key-222",
  }
}"#;
/// 이름이 겹치는 칸이 둘 있고 02-1 해석이 마지막 칸의 값으로 정하는 상태의 파일입니다.
const DUPLICATE_KEY_FILE: &str = r#"{
  /* api_keys 밖 항목 */
  "other_root_key": 1,
  "api_keys": {
    // 중복 설명 주석
    "main": "first-value",
    "main": "last-value",
    "stage_ep": "stage-key-222",
  }
}"#;
const MAIN_KEY_VALUE: &str = "main-key-111";
const OTHER_KEY_VALUE: &str = "stage-key-222";
const OTHER_KEY_TEXT: &str = r#""stage_ep": "stage-key-222""#;
const ROOT_EXTRA_TEXT: &str = r#""other_root_key": 1"#;
