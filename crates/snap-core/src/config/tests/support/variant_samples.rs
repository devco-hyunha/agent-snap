//! 오류 견본 도움: 오류 변형 개수와 변형 이름 되새리기, 표 밖 오류 견본을 한 곳에 모습니다.
//!
//! 설정 가리기 시험(`redaction/`)과 저장 가리기 시험(`save/redaction/`)이 같은 견본을 다시
//! 적지 않게 하려는 자리입니다. 본문은 `tests/redaction.rs`에서 그대로 옮겨 왔습니다.

use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointIssue, FieldIssue, HostOs,
    ModelIssue, ReadingError, SaveError, SaveIoKind, SectionIssue, ValidationError, load_config,
    resolve_config_dir, save_io,
};
use crate::credentials::Credentials;

use super::checks::KEY_VALUE;
use super::config_text::{config_without_kind, config_without_model_id, empty_env, parse_err};

/// `ConfigError`에 걸린 변형의 수입니다. 변형 부분 열거형에 변형을 추가하면
/// `variant_name`의 `match`가 컴파일을 멈춰 알려 주고, 이 값을 함께 올려야 합니다. 올린 뒤에도
/// 견본이 새 변형을 만들지 않으면 아래 테스트가 실패합니다. 목록 하나로 세려면 매크로나 새
/// 의존성이 필요해 이 값 하나만 남깁니다. D29의 `context_window` 값 오류가 견본 표에 늘어 31개,
/// D50 결정 7의 `tool_call_form` 값 오류가 견본 표에 늘어 32개,
/// 03-6의 `agent` 섹션 모르는 항목 오류가 견본 표에 늘어 33개입니다.
pub(crate) const VARIANT_COUNT: usize = 33;

/// 오류 변형 이름입니다. `match`가 감싸는 변형을 부분 열거형까지 갈라 모든 변형을 다뤄야
/// 하므로, 변형이 늘면 컴파일이 멈춥니다.
pub(crate) fn variant_name(error: &ConfigError) -> &'static str {
    match error {
        ConfigError::Reading(error) => match error {
            ReadingError::JsoncSyntax { .. } => "JsoncSyntax",
            ReadingError::RootNotObject => "RootNotObject",
            ReadingError::Io { .. } => "Io",
            ReadingError::ConfigDirUnknown => "ConfigDirUnknown",
            ReadingError::Credentials(_) => "Credentials",
        },
        ConfigError::Validation(error) => match error {
            ValidationError::Endpoint(issue) => match issue {
                // 엔드포인트 항목 규칙 변형입니다.
                EndpointIssue::LegacyTopLevelModels => "LegacyTopLevelModels",
                EndpointIssue::EndpointKeyField { .. } => "EndpointKeyField",
                EndpointIssue::DuplicateEndpoint { .. } => "DuplicateEndpoint",
                EndpointIssue::EmptyEndpointName => "EmptyEndpointName",
                EndpointIssue::EndpointNameHasSlash { .. } => "EndpointNameHasSlash",
                EndpointIssue::InvalidKind { .. } => "InvalidKind",
                EndpointIssue::InvalidBaseUrl { .. } => "InvalidBaseUrl",
            },
            ValidationError::Model(issue) => match issue {
                // 모델 항목 규칙 변형입니다.
                ModelIssue::ModelEndpointField { .. } => "ModelEndpointField",
                ModelIssue::DuplicateModel { .. } => "DuplicateModel",
                ModelIssue::EmptyModelName { .. } => "EmptyModelName",
                ModelIssue::EmptyModelId { .. } => "EmptyModelId",
                ModelIssue::ModelNameHasSlash { .. } => "ModelNameHasSlash",
                ModelIssue::InvalidReasoning { .. } => "InvalidReasoning",
                ModelIssue::InvalidContextWindow { .. } => "InvalidContextWindow",
                ModelIssue::ExtraReservedKey { .. } => "ExtraReservedKey",
                ModelIssue::InvalidToolCallForm { .. } => "InvalidToolCallForm",
            },
            ValidationError::Section(issue) => match issue {
                // 섹션 키 규칙 변형입니다.
                SectionIssue::SamplingOutOfRange { .. } => "SamplingOutOfRange",
                SectionIssue::UnknownSamplingItem { .. } => "UnknownSamplingItem",
                SectionIssue::UnknownHttpItem { .. } => "UnknownHttpItem",
                SectionIssue::UnknownAgentItem { .. } => "UnknownAgentItem",
                SectionIssue::BackoffRangeInverted => "BackoffRangeInverted",
            },
            ValidationError::Field(issue) => match issue {
                // 칸 형식 규칙 변형입니다.
                FieldIssue::InvalidField { .. } => "InvalidField",
                FieldIssue::InvalidEndpointField { .. } => "InvalidEndpointField",
                FieldIssue::InvalidModelField { .. } => "InvalidModelField",
            },
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

/// 거부 사례 표가 만들지 못하는 오류(문법·필수 칸·읽기 실패·폴더 위치·자격 파일)를 만듭니다.
pub(crate) fn errors_outside_rejection_table() -> Vec<ConfigError> {
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
