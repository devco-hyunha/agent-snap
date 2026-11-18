//! 가리기 시험: 오류 메시지와 설정 Debug에서 호스트·키 값이 가려지고 이름만 남는지 확인
//!
//! 가리기 대상 값과 확인 순서, 변형 견본은 지원부를 사용

use std::collections::HashSet;

use super::super::support::{
    HOST, KEY_VALUE, VARIANT_COUNT, assert_error_hides_secrets, base_url, config_of,
    endpoint_object, errors_outside_rejection_table, parse_err, parse_ok, rejection_cases,
    variant_name,
};
use crate::config::{ConfigError, ModelIssue, ReadingError, ValidationError};

/// JSONC 문법 오류 메시지에 줄 번호만 남고 줄 내용과 값은 가려지는지 확인
#[test]
fn jsonc_syntax_error_reports_only_line_number() {
    let text = format!("{{\n  \"endpoints\": [ \"{HOST}\" ]\n  \"x\": \"{KEY_VALUE}\" ]\n}}");

    let error = parse_err(&text);

    assert!(matches!(
        error,
        ConfigError::Reading(ReadingError::JsoncSyntax { line: 3 })
    ));
    assert_error_hides_secrets("문법 오류", &error);
    assert!(error.to_string().contains("(줄 3번)"));
}

/// 설정 Debug에 base_url 호스트가 가려지고 가리기 표기가 남는지 확인
#[test]
fn config_debug_hides_base_url_host() {
    let config = parse_ok(&config_of(&[endpoint_object("main", &base_url(), "")]));
    assert_eq!(config.endpoints[0].base_url.reveal(), base_url());

    let debug = format!("{config:?}");

    assert!(!debug.contains(HOST), "Debug에 호스트가 있습니다: {debug}");
    assert!(debug.contains("가림"));
}

/// `context_window` 값 오류 견본에 호스트·키가 가려지고 모델 이름만 남는지 확인
#[test]
fn context_window_rejection_sample_hides_host_and_key() {
    let case = rejection_cases()
        .into_iter()
        .find(|case| case.label == "context_window가 숫자 아님")
        .expect("거부 사례 표에 `context_window` 숫자 아님 사례가 있어야 합니다");
    let error = parse_err(&case.text);

    assert!(matches!(
        &error,
        ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidContextWindow { model })) if model == "fast"
    ));
    assert!(error.to_string().contains("fast"));
    assert_error_hides_secrets("context_window 숫자 아님", &error);
}

/// 모든 오류 변형의 Display·Debug에서 호스트와 키가 가려지는지, 견본이 모든 변형을 덮는지 확인
#[test]
fn every_error_display_and_debug_hides_host_and_key() {
    let mut errors: Vec<(&str, ConfigError)> = rejection_cases()
        .into_iter()
        .map(|case| (case.label, parse_err(&case.text)))
        .collect();
    errors.extend(
        errors_outside_rejection_table()
            .into_iter()
            .map(|error| ("표 밖 오류", error)),
    );

    for (label, error) in &errors {
        assert_error_hides_secrets(label, error);
    }
    let covered: HashSet<&str> = errors
        .iter()
        .map(|(_, error)| variant_name(error))
        .collect();
    assert_eq!(
        covered.len(),
        VARIANT_COUNT,
        "견본 오류가 모든 변형을 만들어야 합니다: {covered:?}",
    );
}
