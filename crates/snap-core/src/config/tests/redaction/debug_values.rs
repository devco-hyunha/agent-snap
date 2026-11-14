//! 가리기 시험의 이름·값 확인(명세 AC 5 1번째~2번째 줄): 문법 오류와 설정 Debug, `context_window`
//! 견본이 호스트와 키 값 없이 이름만 남기는지 봅니다.
//!
//! 가리기 대상 값과 확인 순서는 지원부(`support/checks.rs`)를, 모든 변형을 세는 견본은
//! 지원부(`support/variant_samples.rs`)를 씁니다.

use std::collections::HashSet;

use super::super::support::{
    HOST, KEY_VALUE, VARIANT_COUNT, assert_error_hides_secrets, base_url, config_of,
    endpoint_object, errors_outside_rejection_table, parse_err, parse_ok, rejection_cases,
    variant_name,
};
use crate::config::{ConfigError, ModelIssue, ReadingError, ValidationError};

/// AC 5(1): JSONC 문법 오류는 줄 번호만 알리고 줄 내용과 값은 없다.
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

/// AC 5(2): 설정의 Debug에는 base_url 호스트가 없고 가리기 표기가 있다.
#[test]
fn config_debug_hides_base_url_host() {
    let config = parse_ok(&config_of(&[endpoint_object("main", &base_url(), "")]));
    assert_eq!(config.endpoints[0].base_url.reveal(), base_url());

    let debug = format!("{config:?}");

    assert!(!debug.contains(HOST), "Debug에 호스트가 있습니다: {debug}");
    assert!(debug.contains("가림"));
}

/// AC 5(2): `context_window` 값 오류 견본이 호스트·키 없이 모델 이름만 담는지 확인한다(D29).
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

/// AC 5(2): 모든 오류 변형의 Display·Debug에 호스트와 키가 없다. 변형 하나도 빠뜨리지 않는다.
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
