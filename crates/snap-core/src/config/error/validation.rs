//! 설정 파일 항목의 검증 규칙 실패 오류를 감싸는 열거형.
//!
//! 표시 규칙은 부분 열거형마다 두고, `ConfigError`까지 한 번에 올리도록 부분 열거형별 `From`을 둄.

mod endpoint;
mod field;
mod model;
mod section;

use std::fmt;

pub use endpoint::EndpointIssue;
pub use field::FieldIssue;
pub use model::ModelIssue;
pub use section::SectionIssue;

use super::ConfigError;

/// 검증 규칙 통과 실패를 네 무리로 나눈 것을 감싸는 열거형.
#[derive(thiserror::Error)]
pub enum ValidationError {
    /// 엔드포인트 항목 규칙에 걸린 실패.
    #[error(transparent)]
    Endpoint(#[from] EndpointIssue),
    /// 모델 항목 규칙에 걸린 실패.
    #[error(transparent)]
    Model(#[from] ModelIssue),
    /// 섹션 키 규칙에 걸린 실패.
    #[error(transparent)]
    Section(#[from] SectionIssue),
    /// 칸 형식 규칙에 걸린 실패.
    #[error(transparent)]
    Field(#[from] FieldIssue),
}

impl From<EndpointIssue> for ConfigError {
    fn from(error: EndpointIssue) -> Self {
        Self::Validation(ValidationError::Endpoint(error))
    }
}

impl From<ModelIssue> for ConfigError {
    fn from(error: ModelIssue) -> Self {
        Self::Validation(ValidationError::Model(error))
    }
}

impl From<SectionIssue> for ConfigError {
    fn from(error: SectionIssue) -> Self {
        Self::Validation(ValidationError::Section(error))
    }
}

impl From<FieldIssue> for ConfigError {
    fn from(error: FieldIssue) -> Self {
        Self::Validation(ValidationError::Field(error))
    }
}

impl ValidationError {
    /// 루트나 `http`·`sampling` 같은 칸의 필수 값이 없거나 타입이 맞지 않을 때의 오류.
    pub(in crate::config) fn invalid_field(field: impl Into<String>) -> Self {
        Self::Field(FieldIssue::InvalidField {
            field: field.into(),
        })
    }

    /// 엔드포인트 항목의 칸이 없거나 타입이 맞지 않을 때의 오류.
    pub(crate) fn invalid_endpoint_field(endpoint: &str, field: impl Into<String>) -> Self {
        Self::Field(FieldIssue::InvalidEndpointField {
            endpoint: endpoint.to_owned(),
            field: field.into(),
        })
    }

    /// 모델 항목의 칸이 없거나 타입이 맞지 않을 때의 오류.
    pub(in crate::config) fn invalid_model_field(
        endpoint: &str,
        model: &str,
        field: impl Into<String>,
    ) -> Self {
        Self::Field(FieldIssue::InvalidModelField {
            endpoint: endpoint.to_owned(),
            model: model.to_owned(),
            field: field.into(),
        })
    }

    /// 모델 이름이 trim 후 비었을 때의 오류.
    pub(in crate::config) fn empty_model_name(endpoint: &str) -> Self {
        Self::Model(ModelIssue::EmptyModelName {
            endpoint: endpoint.to_owned(),
        })
    }

    /// 모델 id가 trim 후 비었을 때의 오류. `model`은 trim한 모델 이름.
    pub(in crate::config) fn empty_model_id(endpoint: &str, model: &str) -> Self {
        Self::Model(ModelIssue::EmptyModelId {
            endpoint: endpoint.to_owned(),
            model: model.to_owned(),
        })
    }
}

impl fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 함.
        write!(f, "ValidationError({})", self)
    }
}
