//! 자격 파일 키 이름 바꾸기·삭제 저장 시험(02-9 작업 3의 자격 쪽 책임).
//!
//! 설정 파일 쪽 이름 바꾸기·삭제 시험은 `config_rename.rs`로 나눕니다. 이 파일은 자격 파일만
//! 다루는 이름 바꾸기·삭제 동작을 봅니다. 자격 파일 텍스트 견본은 자격 저장을 보는 여러 시험이
//! 함께 쓰기 때문에 지원부(`support/credentials.rs`)에서 가져옵니다.

use crate::config::CREDENTIALS_FILE_NAME;
use crate::config::KeyRename;
use crate::config::save::key_rename::{remove_key, rename_key};

use super::super::support::{
    KEYS_ANNOTATED_FILE, KEYS_DUPLICATE_FILE, KEYS_ROOT_EXTRA_TEXT, MAIN_KEY_VALUE, OTHER_KEY_TEXT,
    OTHER_KEY_VALUE, TempDir, assert_error_hides_secrets, count_key_entries, key_value,
    parse_credentials,
};

/// AC 3: 자격 파일의 이름을 바꾸면 키 이름이 새 이름으로 바뀌고 값은 그대로입니다.
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

/// AC 3: 새 이름 키가 이미 있으면 값을 옮기고 이름이 겹치는 칸을 남기지 않습니다.
#[test]
fn renaming_key_onto_existing_target_key_keeps_one_entry_with_moved_value() {
    let folder = TempDir::new("자격-이름-변경-선두");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

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

/// AC 3의 경계: trim한 이름이 같으면 파일을 건드리지 않습니다.
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

/// AC 4: 엔드포인트를 삭제하면 자격 파일의 그 키가 지워집니다.
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

/// AC 4: 그 키가 없으면 자격 파일은 그대로입니다.
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

/// AC 4: 이름이 겹치는 칸이 둘인 파일에서 삭제하면 그 이름 칸을 함께 지웁니다.
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

/// 02-2 이름 규칙에 맞지 않는 이름 바꾸기·삭제는 파일을 건드리지 않고 거부합니다.
/// (이동 전 `invalid_endpoint_name_rejects_before_touching_file`의 이름 바꾸기·삭제 부분)
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
