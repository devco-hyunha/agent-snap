//! 값 가리기 시험(명세 AC 4): 거부 오류에 값은 남지 않고 엔드포인트·모델 이름과 안내만 남는다.
//!
//! AC 4의 두 시험을 담습니다. 둘 다 거부 사례 표를 기준으로 삼으므로 표 계통과 가까운 자리에
//! 함께 두었습니다. 가리기 확인 순서는 지원부(`support/checks.rs`)를 씁니다.

use super::super::support::{
    KEY_VALUE, LEGACY_MODELS_LABEL, MODEL_ENDPOINT_FIELD_LABEL, assert_error_hides_secrets,
    config_with_endpoint_tail, parse_err, rejection_cases,
};
use crate::config::{ConfigError, EndpointIssue, ValidationError};

/// AC 4: `api_key`·`api_key_env` 거부 오류에는 엔드포인트 이름과 안내만 있고 칸 값은 없다.
#[test]
fn key_field_rejection_names_endpoint_and_guides_without_values() {
    for field in ["api_key", "api_key_env"] {
        let text = config_with_endpoint_tail(&format!(r#","{field}":"{KEY_VALUE}""#));

        let error = parse_err(&text);

        assert!(
            matches!(&error, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::EndpointKeyField { endpoint })) if endpoint == "main")
        );
        let display = error.to_string();
        assert!(display.contains("main"));
        assert!(display.contains("credentials.jsonc"));
        assert_error_hides_secrets(field, &error);
    }
}

/// AC 4: 옛 최상위 `models` 거부는 안내만 있고, 모델 `endpoint` 거부는 모델 이름만 있다.
#[test]
fn legacy_field_rejections_carry_guidance_or_model_name_only() {
    let cases = rejection_cases();
    for (label, expected_text) in [
        (LEGACY_MODELS_LABEL, "endpoints[].models"),
        (MODEL_ENDPOINT_FIELD_LABEL, "fast"),
    ] {
        let case = cases
            .iter()
            .find(|case| case.label == label)
            .unwrap_or_else(|| panic!("{label} 사례가 거부 사례 표에 없습니다"));
        let error = parse_err(&case.text);
        assert!(error.to_string().contains(expected_text), "{label}");
        assert_error_hides_secrets(label, &error);
    }
}
