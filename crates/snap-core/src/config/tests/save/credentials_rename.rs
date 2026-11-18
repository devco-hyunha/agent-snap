//! 자격 파일 키 이름 바꾸기·삭제 저장 시험

use crate::config::CREDENTIALS_FILE_NAME;
use crate::config::KeyRename;
use crate::config::save::key_rename::{remove_key, rename_key};

use super::super::support::{
    KEYS_ANNOTATED_FILE, KEYS_DUPLICATE_FILE, KEYS_ROOT_EXTRA_TEXT, MAIN_KEY_VALUE, OTHER_KEY_TEXT,
    OTHER_KEY_VALUE, TempDir, assert_error_hides_secrets, count_key_entries, key_value,
    parse_credentials,
};

/// 자격 파일 키 이름 교체와 값 유지 확인
#[test]
fn renaming_key_moves_the_key_name_and_keeps_the_value() {
    let folder = TempDir::new("자격-이름-변경");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

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
        text.contains(OTHER_KEY_TEXT) && text.contains(KEYS_ROOT_EXTRA_TEXT),
        "주석과 다른 키가 그대로여야 합니다: {text}"
    );
}

/// 새 이름 키가 있는 자리로 옮기고 값 유지, 중복 칸 미생성 확인
#[test]
fn renaming_key_onto_existing_target_key_keeps_one_entry_with_moved_value() {
    let folder = TempDir::new("자격-이름-변경-선두");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

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

/// 이름 중복 칸이 둘인 파일의 이름 바꾸기는 칸 하나로 정리 확인
#[test]
fn renaming_key_over_duplicate_names_collapses_to_one_entry() {
    let folder = TempDir::new("자격-이름-변경-중복");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_DUPLICATE_FILE);

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
    assert_eq!(
        key_value(&credentials, "main-새-이름").as_deref(),
        Some("last-value"),
        "마지막 칸의 값이 그대로 읽혀야 합니다"
    );
    assert_eq!(key_value(&credentials, "main"), None);
}

/// 자격 파일에 그 키가 없으면 이름 바꾸기 미실행과 파일 미변경 확인
#[test]
fn renaming_missing_key_leaves_file_untouched() {
    let folder = TempDir::new("자격-이름-변경-없음");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

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
        KEYS_ANNOTATED_FILE,
        "파일이 그대로여야 합니다"
    );
}

/// 앞뒤 공백을 뺀 이름이 같으면 파일 미변경 확인
#[test]
fn renaming_same_name_leaves_file_untouched() {
    let folder = TempDir::new("자격-이름-변경-동일");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

    rename_key(
        folder.path(),
        &KeyRename {
            from: " main ".to_owned(),
            to: "main ".to_owned(),
        },
    )
    .expect("같은 이름 바꾸기는 성공입니다");

    assert_eq!(
        folder.read(CREDENTIALS_FILE_NAME),
        KEYS_ANNOTATED_FILE,
        "파일이 그대로여야 합니다"
    );
}

/// 엔드포인트 삭제 시 자격 파일의 그 키 삭제 확인
#[test]
fn removing_key_deletes_the_endpoint_key() {
    let folder = TempDir::new("자격-키-삭제");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

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
        text.contains(KEYS_ROOT_EXTRA_TEXT) && text.contains("// 사용자 주석입니다."),
        "주석과 다른 항목이 그대로여야 합니다: {text}"
    );
}

/// 그 키가 없으면 자격 파일 미변경 확인
#[test]
fn removing_missing_key_leaves_file_untouched() {
    let folder = TempDir::new("자격-키-삭제-없음");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

    remove_key(folder.path(), "없는-서버").expect("키가 없는 삭제는 성공입니다");

    assert_eq!(
        folder.read(CREDENTIALS_FILE_NAME),
        KEYS_ANNOTATED_FILE,
        "파일이 그대로여야 합니다"
    );
}

/// 이름 중복 칸이 둘인 파일의 삭제는 그 이름 칸 전체 삭제 확인
#[test]
fn removing_key_over_duplicate_names_removes_all_entries() {
    let folder = TempDir::new("자격-키-삭제-중복");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_DUPLICATE_FILE);

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

/// 이름 규칙 위반 이름 바꾸기·삭제는 파일 미변경, 검증 오류 반환
#[test]
fn invalid_endpoint_name_rejects_rename_and_remove_before_touching_file() {
    for (label, endpoint) in [("빈 이름", "   "), ("슬래시 이름", "a/b")] {
        let folder = TempDir::new("자격-거부");
        folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

        let rename = KeyRename {
            from: endpoint.to_owned(),
            to: "main".to_owned(),
        };
        let error = rename_key(folder.path(), &rename)
            .expect_err(&format!("{label} 이름 바꾸기는 거부되어야 합니다"));
        assert_eq!(
            folder.read(CREDENTIALS_FILE_NAME),
            KEYS_ANNOTATED_FILE,
            "{label}: 이름 바꾸기가 파일을 바꾸었습니다"
        );
        assert_error_hides_secrets(label, &error);
        let error = remove_key(folder.path(), endpoint)
            .expect_err(&format!("{label} 키 삭제가 거부되어야 합니다"));
        assert_eq!(
            folder.read(CREDENTIALS_FILE_NAME),
            KEYS_ANNOTATED_FILE,
            "{label}: 키 삭제가 파일을 바꾸었습니다"
        );
        assert_error_hides_secrets(label, &error);
    }
}
