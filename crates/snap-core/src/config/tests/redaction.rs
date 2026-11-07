//! 오류·Debug 가리기 테스트(명세 AC 4, AC 5): 값은 출력에 남지 않고 이름과 안내만 남는다.

use std::collections::HashSet;

use super::support::{
    HOST, KEY_VALUE, LEGACY_MODELS_LABEL, MODEL_ENDPOINT_FIELD_LABEL, assert_error_hides_secrets,
    base_url, config_of, config_with_endpoint_tail, config_without_kind, config_without_model_id,
    empty_env, endpoint_object, parse_err, parse_ok, rejection_cases,
};
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, HostOs, ReadingError, SaveError,
    SaveIoKind, ValidationError, load_config, resolve_config_dir, save_io,
};
use crate::credentials::Credentials;

const EXAMPLE_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../agent-snap.example.jsonc"
));

/// AC 4: `api_key`·`api_key_env` 거부 오류에는 엔드포인트 이름과 안내만 있고 칸 값은 없다.
#[test]
fn key_field_rejection_names_endpoint_and_guides_without_values() {
    for field in ["api_key", "api_key_env"] {
        let text = config_with_endpoint_tail(&format!(r#","{field}":"{KEY_VALUE}""#));

        let error = parse_err(&text);

        assert!(
            matches!(&error, ConfigError::Validation(ValidationError::EndpointKeyField { endpoint }) if endpoint == "main")
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

/// AC 5(2): 설정의 Debug에는 base_url 호스트가 없고 가림 표기가 있다.
#[test]
fn config_debug_hides_base_url_host() {
    let config = parse_ok(&config_of(&[endpoint_object("main", &base_url(), "")]));
    assert_eq!(config.endpoints[0].base_url.reveal(), base_url());

    let debug = format!("{config:?}");

    assert!(!debug.contains(HOST), "Debug에 호스트가 있습니다: {debug}");
    assert!(debug.contains("가림"));
}

/// `ConfigError`에 걸린 변형의 수입니다. 변형 부분 열거형에 변형을 추가하면
/// `variant_name`의 `match`가 컴파일을 멈춰 알려 주고, 이 값을 함께 올려야 합니다. 올린 뒤에도
/// 견본이 새 변형을 만들지 않으면 아래 테스트가 실패합니다. 목록 하나로 세려면 매크로나 새
/// 의존성이 필요해 이 값 하나만 남깁니다. D29의 `context_window` 값 오류가 견본 표에 늘어 31개입니다.
pub(super) const VARIANT_COUNT: usize = 31;

/// 오류 변형 이름입니다. `match`가 감싸는 변형을 부분 열거형까지 갈라 모든 변형을 다뤄야
/// 하므로, 변형이 늘면 컴파일이 멈춥니다.
pub(super) fn variant_name(error: &ConfigError) -> &'static str {
    match error {
        ConfigError::Reading(error) => match error {
            ReadingError::JsoncSyntax { .. } => "JsoncSyntax",
            ReadingError::RootNotObject => "RootNotObject",
            ReadingError::Io { .. } => "Io",
            ReadingError::ConfigDirUnknown => "ConfigDirUnknown",
            ReadingError::Credentials(_) => "Credentials",
        },
        ConfigError::Validation(error) => match error {
            ValidationError::LegacyTopLevelModels => "LegacyTopLevelModels",
            ValidationError::EndpointKeyField { .. } => "EndpointKeyField",
            ValidationError::ModelEndpointField { .. } => "ModelEndpointField",
            ValidationError::DuplicateEndpoint { .. } => "DuplicateEndpoint",
            ValidationError::DuplicateModel { .. } => "DuplicateModel",
            ValidationError::EmptyEndpointName => "EmptyEndpointName",
            ValidationError::EmptyModelName { .. } => "EmptyModelName",
            ValidationError::EmptyModelId { .. } => "EmptyModelId",
            ValidationError::EndpointNameHasSlash { .. } => "EndpointNameHasSlash",
            ValidationError::ModelNameHasSlash { .. } => "ModelNameHasSlash",
            ValidationError::InvalidKind { .. } => "InvalidKind",
            ValidationError::InvalidBaseUrl { .. } => "InvalidBaseUrl",
            ValidationError::InvalidReasoning { .. } => "InvalidReasoning",
            ValidationError::InvalidContextWindow { .. } => "InvalidContextWindow",
            ValidationError::ExtraReservedKey { .. } => "ExtraReservedKey",
            ValidationError::SamplingOutOfRange { .. } => "SamplingOutOfRange",
            ValidationError::UnknownSamplingItem { .. } => "UnknownSamplingItem",
            ValidationError::UnknownHttpItem { .. } => "UnknownHttpItem",
            ValidationError::BackoffRangeInverted => "BackoffRangeInverted",
            ValidationError::InvalidField { .. } => "InvalidField",
            ValidationError::InvalidEndpointField { .. } => "InvalidEndpointField",
            ValidationError::InvalidModelField { .. } => "InvalidModelField",
        },
        // 어긋난 상태를 밝히는 두 변형은 Display에 주소·키가 없는지 가리기 검사 대상으로만 씁니다.
        ConfigError::Save(error) => match error {
            SaveError::EndpointRenameAfterKeySide { .. } => "EndpointRenameAfterKeySide",
            SaveError::EndpointRemoveAfterKeySide { .. } => "EndpointRemoveAfterKeySide",
            SaveError::SaveIo { .. } => "SaveIo",
            SaveError::MissingEndpoint { .. } => "MissingEndpoint",
        },
    }
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
        ConfigError::Validation(ValidationError::InvalidContextWindow { model }) if model == "fast"
    ));
    assert!(error.to_string().contains("fast"));
    assert_error_hides_secrets("context_window 숫자 아님", &error);
}

/// 거부 사례 표가 만들지 못하는 오류(문법·필수 칸·읽기 실패·폴더 위치·자격 파일)를 만듭니다.
pub(super) fn errors_outside_rejection_table() -> Vec<ConfigError> {
    let syntax = format!("{{\n  \"x\": \"{KEY_VALUE}\" ]\n}}");
    let credentials_error = Credentials::parse(&format!(r#"{{"api_keys":"{KEY_VALUE}"}}"#))
        .expect_err("객체가 아닌 api_keys는 거부됩니다");
    let missing_folder = std::env::temp_dir().join("agent-snap-missing-folder");
    vec![
        parse_err(&syntax),
        parse_err(&config_without_kind()),
        parse_err(&config_without_model_id()),
        load_config(&missing_folder).expect_err("없는 폴더는 오류여야 합니다"),
        resolve_config_dir(&empty_env(HostOs::Windows)).expect_err("정보가 없으면 오류입니다"),
        ConfigError::from(credentials_error),
        // 어긋난 상태 안내는 교체 실패의 원인 없이도(그때는) 만들고, 표시 문구에 값이 없는지 봅니다.
        SaveError::EndpointRenameAfterKeySide { source: None }.into(),
        SaveError::EndpointRemoveAfterKeySide { source: None }.into(),
        // 02-8 저장이 밝히는 파일 쓰기 실패와 대상 엔드포인트 없음은 값 없이 만듭니다.
        save_io(CREDENTIALS_FILE_NAME, SaveIoKind::Replace),
        save_io(CONFIG_FILE_NAME, SaveIoKind::TempWrite),
        ConfigError::missing_endpoint("main"),
    ]
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
        "견본 오류가 모든 변형을 만들어야 합니다: {covered:?}"
    );
}

/// AC 5(3): `agent-snap.example.jsonc`를 읽으면 검증을 통과하고 모델이 들어 있다.
#[test]
fn example_file_passes_validation() {
    let config = parse_ok(EXAMPLE_CONFIG);

    assert_eq!(config.endpoints.len(), 2);
    assert!(config.models().count() >= 2);
}

/// AC 5(3): 예시 파일에는 자리표시자만 있고 실제 키 칸과 알려진 로컬 주소가 없다.
#[test]
fn example_file_contains_placeholders_only() {
    let config = parse_ok(EXAMPLE_CONFIG);

    for endpoint in &config.endpoints {
        assert!(endpoint.name.starts_with('<'), "자리표시자가 아닙니다");
        assert!(endpoint.base_url.reveal().contains("<엔드포인트 호스트"));
    }
    for forbidden in ["localhost", "127.0.0.1", "api_key\"", "sk-"] {
        assert!(
            !EXAMPLE_CONFIG.contains(forbidden),
            "예시에 `{forbidden}`가 있습니다"
        );
    }
}
