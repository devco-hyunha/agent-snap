//! 검증 테스트(명세 AC 2): 거부 사례 표의 모든 항목이 이름만 밝힌 오류로 거부된다.

use std::time::Duration;

use super::support::{
    assert_error_hides_secrets, base_url, config_of, config_with_model_tail, config_with_root_tail,
    config_without_kind, config_without_model_id, endpoint_object, model_object, parse_err,
    parse_ok, rejection_cases,
};
use crate::config::{ConfigError, ValidationError};

/// AC 2: 표의 각 설정은 기대한 오류로 거부되고, 오류에 주소·키 값이 없다.
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

/// AC 2: 경계 값(`temperature` 0과 2, `top_p` 1, `max_tokens` 1)은 통과한다.
#[test]
fn sampling_boundary_values_are_accepted() {
    let config = parse_ok(&config_with_model_tail(
        r#","sampling":{"temperature":0,"top_p":1,"max_tokens":1}"#,
    ));
    let sampling = config.endpoints[0].models[0].sampling;
    assert_eq!(sampling.temperature, Some(0.0));
    assert_eq!(sampling.top_p, Some(1.0));
    assert_eq!(sampling.max_tokens, Some(1));

    let upper = parse_ok(&config_with_model_tail(r#","sampling":{"temperature":2}"#));
    assert_eq!(upper.endpoints[0].models[0].sampling.temperature, Some(2.0));
}

/// AC 2(D21): `http` 시간 값은 아주 작은 양수, `backoff_min == backoff_max`, `retries` 0이 통과한다.
#[test]
fn http_boundary_values_are_accepted() {
    let tiny = parse_ok(&config_with_root_tail(
        r#","http":{"connect_seconds":0.001}"#,
    ));
    assert_eq!(tiny.http.connect_seconds, Duration::from_millis(1));

    let equal = parse_ok(&config_with_root_tail(
        r#","http":{"backoff_min":5,"backoff_max":5}"#,
    ));
    assert_eq!(equal.http.backoff_min, equal.http.backoff_max);

    let no_retry = parse_ok(&config_with_root_tail(r#","http":{"retries":0}"#));
    assert_eq!(no_retry.http.retries, 0);
}

/// AC 2(D21): 시간 항목 6개는 모두 0을 거부하고 오류에는 그 항목 이름만 있다.
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
            matches!(&error, ConfigError::Validation(ValidationError::InvalidField{ field }) if *field == expected),
            "{item}: {error}"
        );
        assert_error_hides_secrets(item, &error);
    }
}

/// AC 2(D21): `backoff_min > backoff_max` 오류는 두 항목 이름만 밝히고 값은 없다.
#[test]
fn inverted_backoff_range_names_both_items_without_values() {
    let error = parse_err(&config_with_root_tail(
        r#","http":{"backoff_min":777,"backoff_max":3}"#,
    ));

    assert!(matches!(
        error,
        ConfigError::Validation(ValidationError::BackoffRangeInverted)
    ));
    let text = format!("{error} {error:?}");
    assert!(text.contains("backoff_min") && text.contains("backoff_max"));
    assert!(!text.contains("777"));
}

/// AC 2: `base_url`은 `/v1` 경로를 검사하지 않고 `http`와 `https`를 모두 받는다.
#[test]
fn base_url_accepts_http_and_https_without_checking_path() {
    for url in ["http://host", "https://host/v1", "HTTP://host/api/v2"] {
        let config = parse_ok(&config_of(&[endpoint_object("main", url, "")]));
        assert_eq!(config.endpoints[0].base_url.reveal(), url);
    }
}

/// AC 2: 필수 칸이 없거나 타입이 틀리면 칸 이름만 밝힌 오류로 거부된다.
#[test]
fn missing_required_fields_are_rejected_with_field_names_only() {
    assert!(matches!(
        parse_err(&config_without_kind()),
        ConfigError::Validation(ValidationError::InvalidEndpointField{ endpoint, field }) if endpoint == "main" && field == "kind"
    ));

    assert!(matches!(
        parse_err(&config_without_model_id()),
        ConfigError::Validation(ValidationError::InvalidModelField{ endpoint, model, field })
            if endpoint == "main" && model == "fast" && field == "id"
    ));

    assert!(matches!(
        parse_err(r#"{"endpoints":{}}"#),
        ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "endpoints"
    ));
}

/// AC 2: 알 수 없는 칸은 루트·엔드포인트·모델 어디에 있어도 무시된다.
#[test]
fn unknown_fields_are_ignored() {
    let model = model_object("fast", "id-1", r#","note":"x""#);
    let endpoint = endpoint_object(
        "main",
        &base_url(),
        &format!(r#","note":1,"models":[{model}]"#),
    );
    let text = format!(r#"{{"note":true,"endpoints":[{endpoint}]}}"#);

    let config = parse_ok(&text);

    assert_eq!(config.endpoints[0].models[0].name, "fast");
}
