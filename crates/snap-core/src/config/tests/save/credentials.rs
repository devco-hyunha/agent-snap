//! 자격 파일 키 추가·수정 저장 시험
//!
//! 자격 파일 텍스트 견본과 키 확인 도움은 지원부(`support/credentials.rs`)에서 가져옴

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

/// 키 저장 시 주석·미수정 항목 보존, 설정 파일 미변경, 왕복 값 확인
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

/// 자격 파일 미존재 시 신규 생성 후 해석 통과 확인
#[test]
fn saving_key_creates_missing_credentials_file() {
    let folder = TempDir::new("자격-새-파일");

    save_key(folder.path(), "main", &SecretValue::new(KEY_VALUE))
        .expect("파일이 없으면 새 파일을 만들어야 합니다");

    let credentials = parse_credentials(&folder.read(CREDENTIALS_FILE_NAME));
    assert_eq!(key_value(&credentials, "main").as_deref(), Some(KEY_VALUE));
}

/// 키 값 없는 수정은 자격 파일 저장 함수 미호출, 원문·왕복 값 유지 확인
#[test]
fn endpoint_only_edit_keeps_existing_key_in_credentials_file() {
    let folder = TempDir::new("자격-기존-키-유지");
    folder.write(CREDENTIALS_FILE_NAME, KEYS_ANNOTATED_FILE);

    let text = folder.read(CREDENTIALS_FILE_NAME);
    assert_eq!(text, KEYS_ANNOTATED_FILE);
    assert_eq!(
        key_value(&parse_credentials(&text), "main").as_deref(),
        Some(MAIN_KEY_VALUE),
        "기존 키 값이 그대로 읽혀야 합니다"
    );
}

/// 이름 규칙 위반은 파일 미변경, 검증 오류 반환
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

/// 해석이 거부하는 파일(최상위 비객체·`api_keys` 비객체·문법 오류)은 쓰기 거부, 파일 유지
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

/// 값이 문자열이 아닌 키는 해석 거부로 편집 경로 생성 전 거부
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

/// 쓰기 실패는 대상 파일 종류와 오류 종류만 밝히고 키 값은 미노출
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

/// 자격 파일 저장은 값이 든 칸만 새 값으로 바꾸고 주석·다른 칸·들여쓰기는 원문 그대로
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
