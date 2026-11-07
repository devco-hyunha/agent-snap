//! 저장 오류 가리기 확인(02-8 작업 5, AC 5): 저장 경로가 내는 모든 오류의 Display·Debug에
//! 키 값과 base_url 호스트가 없음을 모아 확인한다.
//!
//! spec의 '오류 종류와 담는 것' 표 세 줄을 저장 경로에서 만들어 확인한다.
//! - 검증 실패: 02-2와 같은 이름만 담는다(base_url·키는 담지 않는다).
//! - 파일 쓰기·교체 실패: 대상 파일 종류(설정, 자격)와 오류 종류. 경로 전체·키 값·base_url 없음.
//! - 수정·삭제할 엔드포인트 없음: 이름만 담는다.

use std::path::Path;

use super::super::support::{
    HOST, KEY_VALUE, TempDir, assert_error_hides_secrets, base_url, blocking_replacement,
    endpoint_save, side_effect_errors, standard_replacement,
};
use crate::config::save::config::save_endpoint_with;
use crate::config::save::credentials::save_key_with;
use crate::config::save::store::{remove_endpoint_with, rename_endpoint_with};
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointSave, KeyRename, SaveError,
    SaveIoKind, SaveIoTarget, ValidationError,
};
use crate::secrets::SecretValue;

// 검증 거부 입력에 쓰는 주소입니다. `HOST`와 같은 호스트가 들어 있어 Display에 나오면 안 됩니다.
const BAD_URL: &str = "ftp://leak-check-host.invalid/v1";

/// 쓰기·교체 실패를 만드는 저장 경로를 가리키는 타입 이름입니다(clippy의 type_complexity 대응).
type SaveFailure = fn(&Path) -> Result<(), ConfigError>;

/// AC 1: 검증 실패·쓰기 실패·교체 실패·대상 없음 각각의 Display·Debug에 키 값과 base_url
/// 호스트가 없습니다(spec 표 세 줄의 저장 경로 확인).
#[test]
fn each_spec_row_error_hides_key_and_host() {
    let folder = TempDir::new("저장-오류-가리기-전체");
    folder.write(CONFIG_FILE_NAME, HOST_CONFIG);
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_CREDENTIALS);

    for (label, result) in every_save_path_error(folder.path()) {
        let error = result.expect_err(&format!("{label}: 오류가 나야 합니다"));
        assert_error_hides_secrets(label, &error);
    }
}

/// AC 2: 파일 쓰기·교체 실패는 대상 파일 종류와 오류 종류를 밝히고, 경로 전체·키 값·base_url이
/// 없습니다. 설정 쪽과 자격 쪽을 각각 확인합니다.
#[test]
fn write_failure_reports_target_and_kind_without_path() {
    let cases: [(&str, SaveIoTarget, SaveIoKind, SaveFailure); 3] = [
        (
            "설정 교체 실패",
            SaveIoTarget::Config,
            SaveIoKind::Replace,
            config_replace_failure,
        ),
        (
            "자격 교체 실패",
            SaveIoTarget::Credentials,
            SaveIoKind::Replace,
            key_replace_failure,
        ),
        (
            "설정 임시 파일 쓰기 실패",
            SaveIoTarget::Config,
            SaveIoKind::NotFound,
            config_write_failure,
        ),
    ];

    for (label, expected_target, expected_kind, call) in cases {
        let folder = TempDir::new("저장-쓰기-실패-가리기");
        folder.write(CONFIG_FILE_NAME, HOST_CONFIG);
        folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_CREDENTIALS);

        let error = call(folder.path()).expect_err(&format!("{label}: 오류가 나야 합니다"));

        assert!(
            matches!(&error, ConfigError::Save(SaveError::SaveIo{ target, kind })
                if *target == expected_target && *kind == expected_kind),
            "{label}: 대상 파일 종류와 오류 종류여야 합니다: {error}"
        );
        let display = error.to_string();
        assert!(
            display.contains(&expected_target.to_string()),
            "{label}: Display에 대상 파일 종류가 없습니다: {display}"
        );
        assert!(
            display.contains(&expected_kind.to_string()),
            "{label}: Display에 오류 종류가 없습니다: {display}"
        );
        // 경로 조각(파일 이름·폴더 조각)이 Display·Debug에 없습니다.
        for text in [display, format!("{error:?}")] {
            assert!(
                !text.contains(CONFIG_FILE_NAME)
                    && !text.contains(CREDENTIALS_FILE_NAME)
                    && !text.contains(KEY_VALUE)
                    && !text.contains(HOST),
                "{label}: 표시에 경로 조각·값이 있습니다: {text}"
            );
        }
    }
}

/// AC 1(열 확인): 검증 실패는 02-2와 같은 항목 이름만 담고 값은 없습니다.
#[test]
fn validation_failure_carries_names_without_values() {
    let folder = TempDir::new("검증-거부-이름만");
    folder.write(CONFIG_FILE_NAME, HOST_CONFIG);

    let error = save_endpoint_with(folder.path(), &bad_base_url_save(), &replace_file_ok())
        .expect_err("주소 규칙 위반은 거부되어야 합니다");

    assert!(
        matches!(&error, ConfigError::Validation(ValidationError::InvalidBaseUrl{ endpoint }) if endpoint == "main"),
        "02-2와 같은 검증 실패여야 합니다: {error}"
    );
    // Display에 주소가 없고 엔드포인트 이름만 있습니다.
    assert!(
        !error.to_string().contains(HOST) && error.to_string().contains("main"),
        "이름만 담아야 합니다: {error}"
    );
    assert_error_hides_secrets("검증 실패 이름만", &error);
}

/// AC 3: 수정·삭제할 엔드포인트가 없을 때의 오류는 이름만 밝히고 주소·키 값이 없습니다.
#[test]
fn missing_endpoint_error_names_endpoint_only() {
    let folder = TempDir::new("대상-없음-가리기");
    folder.write(CONFIG_FILE_NAME, HOST_CONFIG);
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_CREDENTIALS);

    for label in ["이름 바꾸기 대상 없음", "삭제 대상 없음"] {
        let result = match label {
            "이름 바꾸기 대상 없음" => rename_endpoint_with(
                folder.path(),
                &KeyRename {
                    from: "없는-서버".to_owned(),
                    to: "새-서버".to_owned(),
                },
                &replace_file_ok(),
            ),
            _ => remove_endpoint_with(folder.path(), "없는-서버", &replace_file_ok()),
        };
        let error = result.expect_err(&format!("{label}: 거부되어야 합니다"));
        assert!(
            matches!(&error, ConfigError::Save(SaveError::MissingEndpoint{ endpoint }) if endpoint == "없는-서버"),
            "{label}: 이름만 담는 오류여야 합니다: {error}"
        );
        assert!(
            error.to_string().contains("없는-서버"),
            "{label}: Display에 이름이 없습니다: {error}"
        );
        assert!(
            !error.to_string().contains(&base_url())
                && !error.to_string().contains(HOST)
                && !error.to_string().contains(KEY_VALUE),
            "{label}: Display에 주소·키 값이 있습니다: {error}"
        );
        assert_error_hides_secrets(label, &error);
    }
}

/// AC 5(모음): `ConfigError`의 모든 변형 Display·Debug에 키 값·호스트가 없습니다.
/// 저장 경로가 내는 변형까지 견본에 포함해 변형 하나도 빠뜨리지 않습니다.
#[test]
fn every_config_error_variant_hides_key_and_host() {
    let folder = TempDir::new("전체-변형-가리기");
    folder.write(CONFIG_FILE_NAME, HOST_CONFIG);
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_CREDENTIALS);

    let mut errors: Vec<(&str, ConfigError)> = super::super::support::rejection_cases()
        .into_iter()
        .map(|case| (case.label, super::super::support::parse_err(&case.text)))
        .collect();
    errors.extend(
        super::super::redaction::errors_outside_rejection_table()
            .into_iter()
            .map(|error| ("표 밖 오류", error)),
    );
    errors.extend(
        every_save_path_error(folder.path())
            .into_iter()
            .map(|(label, result)| {
                (
                    label,
                    result.expect_err("견본 오류이므로 저장 오류가 나야 합니다"),
                )
            }),
    );
    // 두 파일을 함께 고칠 때 어긋난 상태를 밝히는 두 변형은 원인 없이도 만듭니다(D40 규칙 3).
    errors.extend(side_effect_errors());

    for (label, error) in &errors {
        assert_error_hides_secrets(label, error);
    }
    let covered: std::collections::HashSet<&str> = errors
        .iter()
        .map(|(_, error)| super::super::redaction::variant_name(error))
        .collect();
    assert_eq!(
        covered.len(),
        super::super::redaction::VARIANT_COUNT,
        "견본 오류가 모든 변형을 만들어야 합니다: {covered:?}"
    );
}

/// AC 5 표 1번째 줄·2번째 줄·3번째 줄의 담는 것을 spec 표대로 확인합니다.
#[test]
fn spec_table_rows_match_redaction_shape() {
    let folder = TempDir::new("spec-표-확인");
    folder.write(CONFIG_FILE_NAME, HOST_CONFIG);
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_CREDENTIALS);

    // 표 1번째 줄: 검증 실패는 02-2와 같은 이름만 담습니다.
    let validation = save_endpoint_with(folder.path(), &bad_base_url_save(), &replace_file_ok())
        .expect_err("주소 규칙 위반");
    assert!(matches!(
        &validation,
        ConfigError::Validation(ValidationError::InvalidBaseUrl { .. })
    ));

    // 표 2번째 줄: 대상 파일 종류와 오류 종류만 담습니다.
    let write_failure = key_replace_failure(folder.path()).expect_err("자격 교체 실패");
    assert!(
        matches!(&write_failure, ConfigError::Save(SaveError::SaveIo{ target, kind })
            if *target == SaveIoTarget::Credentials && *kind == SaveIoKind::Replace),
        "대상 파일 종류와 오류 종류만 담아야 합니다: {write_failure}"
    );

    // 표 3번째 줄: 이름만 담습니다.
    let missing = remove_endpoint_with(folder.path(), "없는-서버", &replace_file_ok())
        .expect_err("대상 없음");
    assert!(
        matches!(&missing, ConfigError::Save(SaveError::MissingEndpoint{ endpoint }) if endpoint == "없는-서버")
    );
}

/// 저장 경로(설정 저장·자격 저장·두 파일 저장)가 내는 오류를 모습니다.
fn every_save_path_error(folder: &Path) -> Vec<(&'static str, Result<(), ConfigError>)> {
    vec![
        (
            "설정 검증 거부",
            save_endpoint_with(folder, &bad_base_url_save(), &replace_file_ok()),
        ),
        (
            "자격 이름 규칙 거부",
            save_key_with(
                folder,
                "a/b",
                &SecretValue::new(KEY_VALUE),
                &replace_file_ok(),
            ),
        ),
        ("설정 교체 실패", config_replace_failure(folder)),
        ("자격 교체 실패", key_replace_failure(folder)),
        (
            "두 파일 이름 바꾸기 교체 실패",
            rename_endpoint_with(
                folder,
                &KeyRename {
                    from: "main".to_owned(),
                    to: "main2".to_owned(),
                },
                &blocking_replacement(CONFIG_FILE_NAME),
            ),
        ),
        (
            "두 파일 삭제 교체 실패",
            remove_endpoint_with(folder, "main", &blocking_replacement(CONFIG_FILE_NAME)),
        ),
        ("설정 쓰기 실패", config_write_failure(folder)),
        (
            "이름 바꾸기 대상 없음",
            rename_endpoint_with(
                folder,
                &KeyRename {
                    from: "없는-서버".to_owned(),
                    to: "새-서버".to_owned(),
                },
                &replace_file_ok(),
            ),
        ),
        (
            "삭제 대상 없음",
            remove_endpoint_with(folder, "없는-서버", &replace_file_ok()),
        ),
    ]
}

/// 규칙에 맞지 않는 주소의 설정 저장 입력입니다. 검증 거부 견본입니다.
fn bad_base_url_save() -> EndpointSave {
    endpoint_save("main", BAD_URL)
}

/// 설정 파일 교체를 막아 교체 실패를 만듭니다.
fn config_replace_failure(folder: &Path) -> Result<(), ConfigError> {
    save_endpoint_with(
        folder,
        &main_save(),
        &blocking_replacement(CONFIG_FILE_NAME),
    )
}

/// 자격 파일 교체를 막아 교체 실패를 만듭니다.
fn key_replace_failure(folder: &Path) -> Result<(), ConfigError> {
    save_key_with(
        folder,
        "main",
        &SecretValue::new(KEY_VALUE),
        &blocking_replacement(CREDENTIALS_FILE_NAME),
    )
}

/// 설정 파일 저장을 없는 하위 폴더에 시도해 임시 파일 쓰기 실패를 만듭니다.
fn config_write_failure(folder: &Path) -> Result<(), ConfigError> {
    save_endpoint_with(
        &folder.join("없는-하위-폴더"),
        &main_save(),
        &replace_file_ok(),
    )
}

/// 검증·대상 없음 경로를 거치게 하는 교체 지점입니다.
fn replace_file_ok() -> impl Fn(&Path, &Path) -> std::io::Result<()> {
    standard_replacement
}

/// 검증·저장 견본으로 쓰는 기본 엔드포인트 저장입니다.
fn main_save() -> EndpointSave {
    endpoint_save("main", &base_url())
}

/// 사용자 주석과 다른 키가 있는 자격 파일 텍스트입니다.
const ANNOTATED_CREDENTIALS: &str = r#"{
  // 사용자 주석입니다.
  "api_keys": { "main": "main-key-111", "stage_ep": "stage-key-222" }
}"#;

/// 가리기 대상 호스트가 든 엔드포인트가 있는 설정 파일 텍스트입니다.
const HOST_CONFIG: &str = r#"{
  // 사용자 주석입니다.
  "endpoints": [{"name": "main", "base_url": "https://leak-check-host.invalid/v1", "kind": "ollama"}]
}"#;
