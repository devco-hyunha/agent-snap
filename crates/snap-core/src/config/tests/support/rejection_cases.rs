//! 거부 사례 표: 명세 AC 2의 거부 항목을 한 줄씩 모아 검증 시험과 가리기 시험이 함께 씁니다.
//!
//! 입력에는 가짜 호스트와 가짜 키 값만 넣고, 기대하는 오류는 닫힌 조건으로 밝힙니다.

use crate::config::{
    ConfigError, EndpointIssue, FieldIssue, ModelIssue, ReadingError, SectionIssue, ValidationError,
};

use super::checks::{HOST, KEY_VALUE};
use super::config_text::{
    base_url, config_of, config_with_endpoint_tail, config_with_model_tail, config_with_models,
    config_with_root_tail, endpoint_object, endpoint_object_with_kind, model_object,
};

/// 사례 라벨: 최상위 옛 `models` 거부입니다. 가리기 검사가 같은 사례를 찾을 때 씁니다.
pub(crate) const LEGACY_MODELS_LABEL: &str = "최상위 옛 models";
/// 사례 라벨: 모델의 `endpoint` 칸 거부입니다. 가리기 검사가 같은 사례를 찾을 때 씁니다.
pub(crate) const MODEL_ENDPOINT_FIELD_LABEL: &str = "모델의 endpoint 칸";

/// 거부 사례 하나입니다. 같은 표를 검증 검사와 가리기 검사가 함께 씁니다.
pub(crate) struct RejectionCase {
    pub(crate) label: &'static str,
    pub(crate) text: String,
    pub(crate) is_expected: Box<dyn Fn(&ConfigError) -> bool>,
}

fn case(
    label: &'static str,
    text: String,
    is_expected: impl Fn(&ConfigError) -> bool + 'static,
) -> RejectionCase {
    RejectionCase {
        label,
        text,
        is_expected: Box::new(is_expected),
    }
}

fn endpoint_with_url(base_url: &str) -> String {
    config_of(&[endpoint_object("main", base_url, "")])
}

/// 모델 `fast`의 샘플링 항목 `item`에 `value`(JSON 값 텍스트)를 적은 범위 위반 사례입니다.
/// 오류가 같은 항목 이름을 밝히는지 확인합니다.
fn sampling_case(label: &'static str, item: &'static str, value: &str) -> RejectionCase {
    case(
        label,
        config_with_model_tail(&format!(r#","sampling":{{"{item}":{value}}}"#)),
        move |error| {
            matches!(error, ConfigError::Validation(ValidationError::Section(SectionIssue::SamplingOutOfRange{ item: found, model }))
                if found == item && model == "fast")
        },
    )
}

/// 모델 `fast`의 `extra`에 예약 키 `key`를 적은 사례입니다. 오류가 같은 키를 밝히는지 확인합니다.
fn reserved_key_case(key: &'static str) -> RejectionCase {
    case(
        "extra 예약 키",
        config_with_model_tail(&format!(r#","extra":{{"{key}":"{KEY_VALUE}"}}"#)),
        move |error| {
            matches!(error, ConfigError::Validation(ValidationError::Model(ModelIssue::ExtraReservedKey{ model, key: found }))
                if model == "fast" && found == key)
        },
    )
}

/// 명세 AC 2의 거부 항목을 한 줄씩 담은 표입니다. 모든 입력에 가짜 호스트가 들어 있습니다.
pub(crate) fn rejection_cases() -> Vec<RejectionCase> {
    let two_main = config_of(&[
        endpoint_object("main", &base_url(), ""),
        endpoint_object("main", &base_url(), ""),
    ]);
    let two_main_trimmed = config_of(&[
        endpoint_object("main", &base_url(), ""),
        endpoint_object("  main  ", &base_url(), ""),
    ]);
    let two_fast = config_with_models(&[
        model_object("fast", "id-1", ""),
        model_object(" fast ", "id-2", ""),
    ]);
    let mut cases = vec![
        case(
            "엔드포인트 이름 중복",
            two_main,
            |e| matches!(e, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::DuplicateEndpoint{ endpoint })) if endpoint == "main"),
        ),
        case(
            "엔드포인트 이름 중복(trim 후)",
            two_main_trimmed,
            |e| matches!(e, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::DuplicateEndpoint{ endpoint })) if endpoint == "main"),
        ),
        case(
            "같은 엔드포인트의 모델 이름 중복",
            two_fast,
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::DuplicateModel{ endpoint, model })) if endpoint == "main" && model == "fast"),
        ),
        case(
            "엔드포인트 api_key 칸",
            config_with_endpoint_tail(&format!(r#","api_key":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::EndpointKeyField{ endpoint })) if endpoint == "main"),
        ),
        case(
            "엔드포인트 api_key_env 칸",
            config_with_endpoint_tail(&format!(r#","api_key_env":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::EndpointKeyField{ endpoint })) if endpoint == "main"),
        ),
        case(
            LEGACY_MODELS_LABEL,
            config_with_root_tail(&format!(
                r#","models":[{{"name":"x","id":"{KEY_VALUE}","endpoint":"main"}}]"#
            )),
            |e| {
                matches!(
                    e,
                    ConfigError::Validation(ValidationError::Endpoint(
                        EndpointIssue::LegacyTopLevelModels
                    ))
                )
            },
        ),
        case(
            MODEL_ENDPOINT_FIELD_LABEL,
            config_with_model_tail(&format!(r#","endpoint":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::ModelEndpointField{ model })) if model == "fast"),
        ),
        case(
            "빈 엔드포인트 이름",
            config_of(&[endpoint_object("   ", &base_url(), "")]),
            |e| {
                matches!(
                    e,
                    ConfigError::Validation(ValidationError::Endpoint(
                        EndpointIssue::EmptyEndpointName
                    ))
                )
            },
        ),
        case(
            "빈 모델 이름",
            config_with_models(&[model_object("  ", "id-1", "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::EmptyModelName{ endpoint })) if endpoint == "main"),
        ),
        case(
            "빈 모델 id",
            config_with_models(&[model_object("fast", "  ", "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::EmptyModelId{ endpoint, model })) if endpoint == "main" && model == "fast"),
        ),
        case(
            "엔드포인트 이름의 슬래시",
            config_of(&[endpoint_object(" a/b ", &base_url(), "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::EndpointNameHasSlash{ endpoint })) if endpoint == "a/b"),
        ),
        case(
            "모델 이름의 슬래시",
            config_with_models(&[model_object("a/b", "id-1", "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::ModelNameHasSlash{ endpoint, model })) if endpoint == "main" && model == "a/b"),
        ),
        case(
            "지원하지 않는 kind",
            config_of(&[endpoint_object_with_kind(
                "main",
                &base_url(),
                KEY_VALUE,
                "",
            )]),
            |e| matches!(e, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::InvalidKind{ endpoint })) if endpoint == "main"),
        ),
        case(
            "지원하지 않는 reasoning",
            config_with_model_tail(&format!(r#","reasoning":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidReasoning{ model })) if model == "fast"),
        ),
        // D29: `context_window`는 1 이상 정수만 받아들입니다. 값은 오류에 밝히지 않습니다.
        case(
            "context_window가 0",
            config_with_model_tail(r#","context_window":0"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidContextWindow{ model })) if model == "fast"),
        ),
        case(
            "context_window가 음수",
            config_with_model_tail(r#","context_window":-1"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidContextWindow{ model })) if model == "fast"),
        ),
        case(
            "context_window가 정수 아님",
            config_with_model_tail(r#","context_window":1.5"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidContextWindow{ model })) if model == "fast"),
        ),
        case(
            "context_window가 숫자 아님",
            config_with_model_tail(&format!(r#","context_window":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidContextWindow{ model })) if model == "fast"),
        ),
        case(
            "context_window가 범위 초과",
            config_with_model_tail(r#","context_window":18446744073709551616"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidContextWindow{ model })) if model == "fast"),
        ),
        // D50 결정 7: `tool_call_form`은 `openai`·`text`·`auto`만 받아들입니다. 값은 오류에 밝히지 않습니다.
        case(
            "tool_call_form이 지원 값 아님",
            config_with_model_tail(&format!(r#","tool_call_form":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidToolCallForm{ model })) if model == "fast"),
        ),
        case(
            "tool_call_form이 문자열 아님",
            config_with_model_tail(r#","tool_call_form":true"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Model(ModelIssue::InvalidToolCallForm{ model })) if model == "fast"),
        ),
        sampling_case("temperature 범위 초과", "temperature", "2.5"),
        sampling_case("temperature 음수", "temperature", "-0.1"),
        sampling_case("top_p가 0", "top_p", "0"),
        sampling_case("top_p가 1 초과", "top_p", "1.01"),
        sampling_case("max_tokens가 0", "max_tokens", "0"),
        sampling_case("max_tokens가 정수 아님", "max_tokens", "1.5"),
        sampling_case("max_tokens가 숫자 아님", "max_tokens", r#""many""#),
        case(
            "모르는 샘플링 항목",
            config_with_model_tail(&format!(r#","sampling":{{"seed":"{KEY_VALUE}"}}"#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Section(SectionIssue::UnknownSamplingItem{ item })) if item == "seed"),
        ),
        case(
            "루트 sampling 범위 초과",
            config_with_root_tail(r#","sampling":{"top_p":5}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Section(SectionIssue::SamplingOutOfRange{ item, model })) if item == "top_p" && model == "기본"),
        ),
        case(
            "모르는 http 항목(옛 이름)",
            config_with_root_tail(&format!(r#","http":{{"max_retries":"{KEY_VALUE}"}}"#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Section(SectionIssue::UnknownHttpItem{ item })) if item == "max_retries"),
        ),
        case(
            "http 값이 음수",
            config_with_root_tail(r#","http":{"connect_seconds":-1}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "http.connect_seconds"),
        ),
        case(
            "http 값이 0",
            config_with_root_tail(r#","http":{"connect_seconds":0}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "http.connect_seconds"),
        ),
        case(
            "http 값이 무한대",
            config_with_root_tail(r#","http":{"stream_idle_seconds":1e999}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "http.stream_idle_seconds"),
        ),
        case(
            "http 값이 나노초 미만(0으로 취급)",
            config_with_root_tail(r#","http":{"connect_seconds":1e-10}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "http.connect_seconds"),
        ),
        case(
            "http retries가 음수",
            config_with_root_tail(r#","http":{"retries":-1}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "http.retries"),
        ),
        case(
            "http 값이 숫자 아님",
            config_with_root_tail(&format!(r#","http":{{"backoff_min":"{KEY_VALUE}"}}"#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "http.backoff_min"),
        ),
        case(
            "backoff_min이 backoff_max보다 큼",
            config_with_root_tail(r#","http":{"backoff_min":40}"#),
            |e| {
                matches!(
                    e,
                    ConfigError::Validation(ValidationError::Section(
                        SectionIssue::BackoffRangeInverted
                    ))
                )
            },
        ),
        case(
            "최상위가 객체가 아님",
            format!(r#"["{HOST}"]"#),
            |e| matches!(e, ConfigError::Reading(ReadingError::RootNotObject)),
        ),
        case("빈 파일", String::new(), |e| {
            matches!(e, ConfigError::Reading(ReadingError::RootNotObject))
        }),
        case(
            "http retries가 정수 아님",
            config_with_root_tail(r#","http":{"retries":1.5}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField{ field })) if field == "http.retries"),
        ),
        case(
            "모르는 agent 항목",
            config_with_root_tail(r#","agent":{"idle_timeout":9}"#),
            |e| {
                matches!(
                    e,
                    ConfigError::Validation(ValidationError::Section(SectionIssue::UnknownAgentItem{ item }))
                        if item == "idle_timeout"
                )
            },
        ),
    ];
    // 명세 AC 2의 기대값이라 구현 상수와 따로 적습니다(구현 상수를 쓰면 순환 검증이 됩니다).
    cases.extend(
        ["model", "messages", "stream", "tools"]
            .into_iter()
            .map(reserved_key_case),
    );
    for (label, url) in [
        ("base_url scheme이 ftp", format!("ftp://{HOST}/v1")),
        ("base_url에 scheme 없음", format!("{HOST}/v1")),
        ("base_url이 빈 문자열", String::new()),
        ("base_url 앞 공백", format!(" https://{HOST}/v1")),
        ("base_url 뒤 공백", format!("https://{HOST}/v1 ")),
        ("base_url 끝 슬래시", format!("https://{HOST}/v1/")),
    ] {
        cases.push(case(
            label,
            endpoint_with_url(&url),
            |e| matches!(e, ConfigError::Validation(ValidationError::Endpoint(EndpointIssue::InvalidBaseUrl{ endpoint })) if endpoint == "main"),
        ));
    }
    cases
}
