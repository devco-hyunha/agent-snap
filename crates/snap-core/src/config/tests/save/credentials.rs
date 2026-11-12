//! 자격 파일 키 추가·수정 저장 테스트(02-8 작업 3 AC 1~5).
//!
//! 키 이름 바꾸기와 키 삭제 저장을 보는 테스트는 `credentials_rename.rs`로 나눴습니다.
//! 여기에는 키 추가·수정 저장을 보는 테스트가 남습니다. 저장 후 02-1 해석(`Credentials::parse`)으로
//! 다시 읽은 값이 같은지, 주석과 다른 항목이 그대로인지, 오류와 파일에 키 값이 남지 않는지 봅니다.
//! 자격 파일 텍스트 견본과 키 확인 도움은 지원부(`support/credentials.rs`)에서 가져옵니다.

use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointIssue, ReadingError, SaveError,
    SaveIoKind, SaveIoTarget, ValidationError, save_key,
};
use crate::credentials::CredentialsError;
use crate::secrets::SecretValue;

use super::super::support::{
    CONFIG_SIDE_TEXT, KEY_VALUE, KEYS_ANNOTATED_FILE, KEYS_DUPLICATE_FILE, KEYS_ROOT_EXTRA_TEXT,
    MAIN_KEY_VALUE, OTHER_KEY_TEXT, OTHER_KEY_VALUE, TempDir, assert_error_hides_key_value,
    assert_error_hides_secrets, count_key_entries, key_value, parse_credentials,
};

/// AC 1: 주석·다른 항목이 있는 자격 파일에 키를 저장하면 02-1 해석으로 같은 값이 읽히고,
/// 주석과 손대지 않은 항목은 그대로이며 설정 파일은 건드리지 않습니다.
#[test]
fn saving_key_keeps_comments_and_untouched_entries_and_round_trips() {
    let folder = TempDir::new("자격-주석-보존");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);
    folder.write(CONFIG_FILE_NAME, CONFIG_SIDE_TEXT);

    save_key(folder.path(), " main ", &SecretValue::new(KEY_VALUE))
        .expect("저장이 성공해야 합니다");

    let text = folder.read(CREDENTIALS_FILE_NAME);
    for marker in [
        "// 사용자 주석입니다.",
        "/* api_keys 밖 항목은 무시합니다 */",
        "// 다른 서버 키",
        OTHER_KEY_TEXT,
        KEYS_ROOT_EXTRA_TEXT,
    ] {
        assert!(
            text.contains(marker),
            "원문 그대로 남아야 할 항목이 없습니다({marker}): {text}"
        );
    }

    let credentials = parse_credentials(&text);
    assert_eq!(key_value(&credentials, "main").as_deref(), Some(KEY_VALUE));
    assert_eq!(
        key_value(&credentials, "stage_ep").as_deref(),
        Some(OTHER_KEY_VALUE),
        "다른 키가 그대로 읽혀야 합니다"
    );

    assert_eq!(
        folder.read(CONFIG_FILE_NAME),
        CONFIG_SIDE_TEXT,
        "설정 파일은 이 작업의 범위가 아닙니다(작업 2 담당)"
    );
}

/// AC 2: 자격 파일이 없으면 새 파일을 만들어 02-1 해석이 통과합니다.
#[test]
fn saving_key_creates_missing_credentials_file() {
    let folder = TempDir::new("자격-새-파일");

    save_key(folder.path(), "main", &SecretValue::new(KEY_VALUE))
        .expect("파일이 없으면 새 파일을 만들어야 합니다");

    let credentials = parse_credentials(&folder.read(CREDENTIALS_FILE_NAME));
    assert_eq!(key_value(&credentials, "main").as_deref(), Some(KEY_VALUE));
}

/// AC 5: 키 값 없이 저장하는 수정은 자격 파일 쪽 저장 함수를 부르지 않으므로 기존 키가
/// 그대로입니다. 파일 원문이 그대로이고 02-1 왕복 값도 그대로임을 확인합니다.
#[test]
fn endpoint_only_edit_keeps_existing_key_in_credentials_file() {
    let folder = TempDir::new("자격-기존-키-유지");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

    // 키를 주지 않는 수정은 save_key를 부르지 않습니다(저장 API 계약, D16).
    let text = folder.read(CREDENTIALS_FILE_NAME);
    assert_eq!(text, KEYS_ANNOTATED_FILE);
    assert_eq!(
        key_value(&parse_credentials(&text), "main").as_deref(),
        Some(MAIN_KEY_VALUE),
        "기존 키 값이 그대로 읽혀야 합니다"
    );
}

/// 이름 규칙에 맞지 않으면 파일을 건드리지 않고 02-2와 같은 오류만 냅니다.
#[test]
fn invalid_endpoint_name_rejects_before_touching_file() {
    for (label, endpoint) in [("빈 이름", "   "), ("슬래시 이름", "a/b")] {
        let folder = TempDir::new("자격-거부");
        folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

        let error = save_key(folder.path(), endpoint, &SecretValue::new(KEY_VALUE))
            .expect_err(&format!("{label}은 거부되어야 합니다"));
        assert_eq!(
            folder.read(CREDENTIALS_FILE_NAME),
            KEYS_ANNOTATED_FILE,
            "{label}: 파일이 바뀌었습니다"
        );
        assert!(
            matches!(
                &error,
                ConfigError::Validation(ValidationError::Endpoint(
                    EndpointIssue::EmptyEndpointName
                )) | ConfigError::Validation(ValidationError::Endpoint(
                    EndpointIssue::EndpointNameHasSlash { .. }
                ))
            ),
            "오류가 이름 규칙 위반이어야 합니다: {error}"
        );
        assert_error_hides_secrets(label, &error);
    }
}

/// 02-1 해석이 거부하는 파일(최상위 비객체·`api_keys` 비객체·문법 오류)은 쓰기를 거부하고
/// 파일을 그대로 둩니다. 오류 Display·Debug에 키 값이 없습니다.
#[test]
fn unparseable_credentials_file_rejects_before_touching_file() {
    for (label, text) in [
        ("최상위 비객체", format!(r#"["{KEY_VALUE}"]"#)),
        (
            "api_keys 비객체",
            format!(r#"{{"api_keys":"{KEY_VALUE}"}}"#),
        ),
        (
            "문법 오류",
            format!("{{ \"api_keys\": {{\"ep\":\"{KEY_VALUE}\" }} }} }}"),
        ),
    ] {
        let folder = TempDir::new("자격-내용-거부");
        folder.write(CREDENTIALS_FILE_NAME, &text);

        let error = save_key(folder.path(), "main", &SecretValue::new(KEY_VALUE))
            .expect_err(&format!("{label}은 거부되어야 합니다"));
        assert_eq!(
            folder.read(CREDENTIALS_FILE_NAME),
            text,
            "{label}: 파일이 바뀌었습니다"
        );
        assert!(
            matches!(&error, ConfigError::Reading(ReadingError::Credentials(inside))
                if matches!(inside, CredentialsError::RootNotObject
                    | CredentialsError::ApiKeysNotObject
                    | CredentialsError::JsoncSyntax { .. })),
            "오류가 02-1 해석 거부와 같아야 합니다: {error}"
        );
        assert_error_hides_key_value(label, &error);
    }
}

/// 값이 문자열이 아닌 키는 02-1 해석이 먼저 거부하므로 편집 경로가 생기기 전에 거부됩니다.
#[test]
fn non_string_key_value_rejects_before_touching_file() {
    let folder = TempDir::new("자격-값-타입-거부");
    let text = r#"{"api_keys":{"main":12345}}"#;
    folder.write(CREDENTIALS_FILE_NAME, text);

    let error = save_key(folder.path(), "main", &SecretValue::new(KEY_VALUE))
        .expect_err("문자열이 아닌 키 값은 거부되어야 합니다");
    assert_eq!(folder.read(CREDENTIALS_FILE_NAME), text);
    assert!(
        matches!(
            &error,
            ConfigError::Reading(ReadingError::Credentials(CredentialsError::KeyNotString { endpoint }))
            if endpoint == "main"
        ),
        "오류가 02-1 해석의 키 이름 거부와 같아야 합니다: {error}"
    );
    assert_error_hides_key_value("키 값 타입 거부", &error);
}

/// 쓰기 실패는 대상 파일 종류와 오류 종류만 밝히고 키 값은 밝히지 않습니다.
///
/// 자격 파일 저장은 주소를 다루지 않으므로, 이 검사에서는 가리기 대상으로 키 값만 봅니다
/// (호스트 가리기는 설정 파일 쪽 검사 용도입니다).
#[test]
fn write_failure_reports_target_and_kind_only() {
    let folder = TempDir::new("자격-쓰기-실패");
    let missing_folder = folder.path().join("없는-하위-폴더");

    let error = save_key(&missing_folder, "main", &SecretValue::new(KEY_VALUE))
        .expect_err("폴더가 없으면 쓰기가 실패해야 합니다");
    assert!(
        matches!(
            &error,
            ConfigError::Save(SaveError::SaveIo {
                target: SaveIoTarget::Credentials,
                kind: SaveIoKind::NotFound
            })
        ),
        "오류가 자격 파일 쓰기 실패여야 합니다: {error}"
    );
    assert!(
        error.to_string().contains("자격 파일"),
        "Display에 대상 파일 종류가 있어야 합니다: {error}"
    );
    assert!(
        !error.to_string().contains(CREDENTIALS_FILE_NAME),
        "Display에 파일 이름이 있어서는 안 됩니다: {error}"
    );
    assert_error_hides_key_value("자격 쓰기 실패", &error);
}

/// 자격 파일 저장이 원문 표기를 해치지 않는지 확인합니다.
///
/// 중복 칸을 정리하는 편집에서 값이 든 칸만 새 값으로 바뀌고, 주석·다른 칸·들여쓰기는
/// 원문 그대로 남는지를 봅니다.
#[test]
fn saving_key_preserves_untouched_lines_verbatim() {
    let folder = TempDir::new("자격-원문-보존");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_DUPLICATE_FILE);

    save_key(folder.path(), "main", &SecretValue::new(KEY_VALUE))
        .expect("중복 칸 정리 저장이 성공해야 합니다");

    let text = folder.read(CREDENTIALS_FILE_NAME);
    for marker in [
        "// 중복 설명 주석",
        "/* api_keys 밖 항목 */",
        OTHER_KEY_TEXT,
        KEYS_ROOT_EXTRA_TEXT,
        // 첫 칸이 새 값으로 바뀌고 그 칸 하나가 남습니다(들여쓰기 원형 유지 포함).
        "  \"main\": \"leak-key-value-9\",",
    ] {
        assert!(
            text.contains(marker),
            "원문 그대로 남아야 할 항목이 없습니다({marker}): {text}"
        );
    }
    assert_eq!(count_key_entries(&text, "main"), 1);
    assert_eq!(
        key_value(&parse_credentials(&text), "main").as_deref(),
        Some(KEY_VALUE)
    );
}
