//! 칸 형식 검증(명세 AC 2): 필수 칸·모르는 칸의 처리를 본다.

use super::super::support::{
    base_url, config_without_kind, config_without_model_id, endpoint_object, model_object,
    parse_err, parse_ok,
};
use crate::config::{ConfigError, FieldIssue, ValidationError};

/// AC 2: 필수 칸이 없거나 타입이 틀리면 칸 이름만 밝힌 오류로 거부된다.
#[test]
fn missing_required_fields_are_rejected_with_field_names_only() {
    assert!(matches!(
        parse_err(&config_without_kind()),
        ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidEndpointField{ endpoint, field })) if endpoint == "main" && field == "kind"
    ));

    assert!(matches!(
        parse_err(&config_without_model_id()),
        ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidModelField{ endpoint, model, field }))
            if endpoint == "main" && model == "fast" && field == "id"
    ));

    assert!(matches!(
        parse_err(r#"{"endpoints":{}}"#),
        ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "endpoints"
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
