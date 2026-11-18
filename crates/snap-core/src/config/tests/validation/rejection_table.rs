//! 거부 사례 표 검증: 표 항목과 표와 같은 성격의 거부를 확인
//!
//! 표는 지원부(`support::rejection_cases`)가 만들었고, 표가 만들지 않는 거부는 여기서 직접 확인

use super::super::support::{
    assert_error_hides_secrets, config_with_model_tail, config_with_root_tail, parse_err,
    rejection_cases,
};
use crate::config::{ConfigError, FieldIssue, ModelIssue, SectionIssue, ValidationError};

/// 표의 각 설정은 기대한 오류로 거부되고, 오류에 주소·키 값은 없음
#[test]
fn every_rejection_case_is_rejected_with_the_expected_error_without_values() {
    for case in rejection_cases() {
        let error = parse_err(&case.text);
        assert!(
            (case.is_expected)(&error),
            "{}: 기대와 다른 오류입니다: {error}",
            case.label
        );
        assert_error_hides_secrets(case.label, &error);
    }
}

/// `context_window` 거부는 모델 이름만 밝히고 입력 값은 밝히지 않음
#[test]
fn context_window_rejection_names_model_without_the_value() {
    // 값 0은 규칙 위반이라 거부되고, 오류에 0이 나오지 않는 것을 보는 확인용 값
    let error = parse_err(&config_with_model_tail(r#","context_window":0"#));

    assert!(matches!(
        &error,
        ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidContextWindow { model })) if model == "fast"
    ));
    // Display·Debug에 입력 값은 없고 모델 이름만 있는지 확인
    let text = format!("{error} {error:?}");
    assert!(
        text.contains("context_window"),
        "칸 이름이 있어야 합니다: {text}"
    );
    assert!(text.contains("fast"), "모델 이름이 있어야 합니다: {text}");
    assert!(!text.contains(":0"), "입력 값이 있습니다: {text}");
    assert_error_hides_secrets("context_window 값 오류", &error);
}

/// 시간 항목 6개는 모두 0을 거부하고 오류에는 그 항목 이름만 있음
#[test]
fn every_http_time_item_rejects_zero_with_item_name_only() {
    let items = [
        "backoff_min",
        "backoff_max",
        "retry_after_max",
        "connect_seconds",
        "header_wait_seconds",
        "stream_idle_seconds",
    ];
    for item in items {
        let error = parse_err(&config_with_root_tail(&format!(
            r#","http":{{"{item}":0}}"#
        )));

        let expected = format!("http.{item}");
        assert!(
            matches!(&error, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if *field == expected),
            "{item}: {error}"
        );
        assert_error_hides_secrets(item, &error);
    }
}

/// `backoff_min > backoff_max` 오류는 두 항목 이름만 밝히고 값은 없음
#[test]
fn inverted_backoff_range_names_both_items_without_values() {
    let error = parse_err(&config_with_root_tail(
        r#","http":{"backoff_min":777,"backoff_max":3}"#,
    ));

    assert!(matches!(
        error,
        ConfigError::Validation(ValidationError::Section(SectionIssue::BackoffRangeInverted))
    ));
    let text = format!("{error} {error:?}");
    assert!(text.contains("backoff_min") && text.contains("backoff_max"));
    assert!(!text.contains("777"));
}
