//! 오류 견본 도움: 오류 변형 개수와 변형 이름 되새리기, 표 밖 오류 견본 목록.
//!
//! 설정 가리기 시험과 저장 가리기 시험이 같은 견본을 함께 쓰는 제약.

use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointIssue, FieldIssue, HostOs,
    ModelIssue, ReadingError, SaveError, SaveIoKind, SectionIssue, ValidationError, load_config,
    resolve_config_dir, save_io,
};
use crate::credentials::Credentials;

use super::checks::KEY_VALUE;
use super::config_text::{config_without_kind, config_without_model_id, empty_env, parse_err};

/// `ConfigError`의 변형 수.
pub(crate) const VARIANT_COUNT: usize = 33;

/// 오류 변형 이름을 되새겨 반환.
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
                EndpointIssue::LegacyTopLevelModels => "LegacyTopLevelModels",
                EndpointIssue::EndpointKeyField { .. } => "EndpointKeyField",
                EndpointIssue::DuplicateEndpoint { .. } => "DuplicateEndpoint",
                EndpointIssue::EmptyEndpointName => "EmptyEndpointName",
                EndpointIssue::EndpointNameHasSlash { .. } => "EndpointNameHasSlash",
                EndpointIssue::InvalidKind { .. } => "InvalidKind",
                EndpointIssue::InvalidBaseUrl { .. } => "InvalidBaseUrl",
            },
            ValidationError::Model(issue) => match issue {
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
                SectionIssue::SamplingOutOfRange { .. } => "SamplingOutOfRange",
                SectionIssue::UnknownSamplingItem { .. } => "UnknownSamplingItem",
                SectionIssue::UnknownHttpItem { .. } => "UnknownHttpItem",
                SectionIssue::UnknownAgentItem { .. } => "UnknownAgentItem",
                SectionIssue::BackoffRangeInverted => "BackoffRangeInverted",
            },
            ValidationError::Field(issue) => match issue {
                FieldIssue::InvalidField { .. } => "InvalidField",
                FieldIssue::InvalidEndpointField { .. } => "InvalidEndpointField",
                FieldIssue::InvalidModelField { .. } => "InvalidModelField",
            },
        },
        // 어긋난 상태를 밝히는 두 변형은 Display에 주소·키가 없는지 가리기 검사 대상으로만 씀.
        ConfigError::Save(error) => match error {
            SaveError::EndpointRenameAfterKeySide { .. } => "EndpointRenameAfterKeySide",
            SaveError::EndpointRemoveAfterKeySide { .. } => "EndpointRemoveAfterKeySide",
            SaveError::SaveIo { .. } => "SaveIo",
            SaveError::MissingEndpoint { .. } => "MissingEndpoint",
        },
    }
}

/// 거부 사례 표가 만들지 못하는 오류를 만드는 견본 목록.
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
        // 어긋난 상태는 교체 실패의 원인 없이 만들고, 표시 문구에 값이 없는지 확인.
        SaveError::EndpointRenameAfterKeySide { source: None }.into(),
        SaveError::EndpointRemoveAfterKeySide { source: None }.into(),
        // 쓰기 실패와 대상 엔드포인트 없음은 값 없이 만듦.
        save_io(CREDENTIALS_FILE_NAME, SaveIoKind::Replace),
        save_io(CONFIG_FILE_NAME, SaveIoKind::TempWrite),
        ConfigError::missing_endpoint("main"),
    ]
}
