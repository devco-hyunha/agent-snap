//! 경계값 통과 검증(명세 AC 2): 규칙의 가장자리 값이 통과하는지 본다.

use super::super::support::{
    config_of, config_with_model_tail, config_with_root_tail, endpoint_object, parse_ok,
};

/// AC 2(D29): `context_window`는 경계 값 1과 `u64` 상단이 통과한다.
#[test]
fn context_window_boundary_values_are_accepted() {
    let low = parse_ok(&config_with_model_tail(r#","context_window":1"#));
    assert_eq!(low.endpoints[0].models[0].context_window, 1);

    let max = parse_ok(&config_with_model_tail(
        r#","context_window":18446744073709551615"#,
    ));
    assert_eq!(max.endpoints[0].models[0].context_window, u64::MAX);
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
    assert_eq!(
        tiny.http.connect_seconds,
        std::time::Duration::from_millis(1)
    );

    let equal = parse_ok(&config_with_root_tail(
        r#","http":{"backoff_min":5,"backoff_max":5}"#,
    ));
    assert_eq!(equal.http.backoff_min, equal.http.backoff_max);

    let no_retry = parse_ok(&config_with_root_tail(r#","http":{"retries":0}"#));
    assert_eq!(no_retry.http.retries, 0);
}

/// AC 2: `base_url`은 `/v1` 경로를 검사하지 않고 `http`와 `https`를 모두 받는다.
#[test]
fn base_url_accepts_http_and_https_without_checking_path() {
    for url in ["http://host", "https://host/v1", "HTTP://host/api/v2"] {
        let config = parse_ok(&config_of(&[endpoint_object("main", url, "")]));
        assert_eq!(config.endpoints[0].base_url.reveal(), url);
    }
}
