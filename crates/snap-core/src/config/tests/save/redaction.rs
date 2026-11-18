//! 저장 경로 오류의 Display·Debug에서 키 값·base_url 호스트 가리기 확인
//!
//! 오류 견본과 저장 경로 조립은 지원부(`support/save_failures.rs`)에서 함께 씀

use std::path::Path;

use super::super::support::{
    ANNOTATED_CREDENTIALS, HOST, HOST_CONFIG, KEY_VALUE, TempDir, assert_error_hides_secrets,
    bad_base_url_save, base_url, config_replace_failure, config_write_failure,
    every_save_path_error, key_replace_failure, replace_file_ok, side_effect_errors,
};
use crate::config::save::config::save_endpoint_with;
use crate::config::save::store::{remove_endpoint_with, rename_endpoint_with};
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointIssue, KeyRename, SaveError,
    SaveIoKind, SaveIoTarget, ValidationError,
};

/// 쓰기·교체 실패 저장 경로를 가리키는 타입 이름
type SaveFailure = fn(&Path) -> Result<(), ConfigError>;

/// 저장 경로 오류 전체의 키 값·호스트 미포함 확인
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

/// 쓰기·교체 실패는 대상 파일 종류와 오류 종류만 밝힘. 경로 전체·키 값·base_url 미포함
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

/// 검증 실패는 항목 이름만 담고 값은 미포함 확인
#[test]
fn validation_failure_carries_names_without_values() {
    let folder = TempDir::new("검증-거부-이름만");
    folder.write(CONFIG_FILE_NAME, HOST_CONFIG);

    let error = save_endpoint_with(folder.path(), &bad_base_url_save(), &replace_file_ok())
        .expect_err("주소 규칙 위반은 거부되어야 합니다");

    assert!(
        matches!(&error, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::InvalidBaseUrl{ endpoint })) if endpoint == "main"),
        "02-2와 같은 검증 실패여야 합니다: {error}"
    );
    assert!(
        !error.to_string().contains(HOST) && error.to_string().contains("main"),
        "이름만 담아야 합니다: {error}"
    );
    assert_error_hides_secrets("검증 실패 이름만", &error);
}

/// 대상 엔드포인트 없음 오류는 이름만 밝히고 주소·키 값 미포함 확인
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

/// `ConfigError` 모든 변형의 Display·Debug에서 키 값·호스트 미포함
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
        super::super::support::errors_outside_rejection_table()
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
    errors.extend(side_effect_errors());

    for (label, error) in &errors {
        assert_error_hides_secrets(label, error);
    }
    let covered: std::collections::HashSet<&str> = errors
        .iter()
        .map(|(_, error)| super::super::support::variant_name(error))
        .collect();
    assert_eq!(
        covered.len(),
        super::super::support::VARIANT_COUNT,
        "견본 오류가 모든 변형을 만들어야 합니다: {covered:?}"
    );
}

/// 검증 실패·쓰기 실패·대상 없음 세 경우의 담는 형태 확인
#[test]
fn spec_table_rows_match_redaction_shape() {
    let folder = TempDir::new("spec-표-확인");
    folder.write(CONFIG_FILE_NAME, HOST_CONFIG);
    folder.write(CREDENTIALS_FILE_NAME, ANNOTATED_CREDENTIALS);

    let validation = save_endpoint_with(folder.path(), &bad_base_url_save(), &replace_file_ok())
        .expect_err("주소 규칙 위반");
    assert!(matches!(
        &validation,
        ConfigError::Validation(ValidationError::Endpoint(
            EndpointIssue::InvalidBaseUrl { .. }
        ))
    ));

    let write_failure = key_replace_failure(folder.path()).expect_err("자격 교체 실패");
    assert!(
        matches!(&write_failure, ConfigError::Save(SaveError::SaveIo{ target, kind })
            if *target == SaveIoTarget::Credentials && *kind == SaveIoKind::Replace),
        "대상 파일 종류와 오류 종류만 담아야 합니다: {write_failure}"
    );

    let missing = remove_endpoint_with(folder.path(), "없는-서버", &replace_file_ok())
        .expect_err("대상 없음");
    assert!(
        matches!(&missing, ConfigError::Save(SaveError::MissingEndpoint{ endpoint }) if endpoint == "없는-서버")
    );
}
