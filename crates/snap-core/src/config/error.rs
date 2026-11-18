//! 설정 읽기·검증·저장 오류를 세 부분 열거형으로 감싸는 열거형.
//!
//! 호출부가 `ConfigError` 경로만 쓰는 제약.

mod reading;
mod save;
mod validation;

pub use reading::ReadingError;
pub use save::SaveError;
pub use validation::{EndpointIssue, FieldIssue, ModelIssue, SectionIssue, ValidationError};

use std::fmt;

use crate::credentials::CredentialsError;
use crate::jsonc::JsoncError;

/// 설정 파일 읽기·검증·저장 실패 오류.
///
/// 변경 이유가 서로 다른 셋(읽기·검증·저장)을 각각의 부분 열거형으로 놓고, 이 타입은 감싸기만
/// 함. 표시 규칙은 부분 열거형마다 둠.
#[derive(thiserror::Error)]
pub enum ConfigError {
    /// 파일을 찾고 읽고 해석하는 단계의 실패.
    #[error(transparent)]
    Reading(#[from] ReadingError),
    /// 채운 값이 검증 규칙을 통과하지 못한 실패.
    #[error(transparent)]
    Validation(#[from] ValidationError),
    /// 파일을 쓰는 중의 실패.
    #[error(transparent)]
    Save(#[from] SaveError),
}

impl From<JsoncError> for ConfigError {
    fn from(error: JsoncError) -> Self {
        Self::Reading(ReadingError::from(error))
    }
}

impl From<CredentialsError> for ConfigError {
    fn from(error: CredentialsError) -> Self {
        Self::Reading(ReadingError::from(error))
    }
}

impl ConfigError {
    // 부분 열거형의 같은 생성 함수를 공개 경로 이름으로 이어 주는 도움 함수.
    // 새 생성 함수는 부분 열거형의 대응 자리에 먼저 만들고, 공개 경로를 쓸 때만 이곳에 더함.
    pub(super) fn missing_endpoint(endpoint: &str) -> Self {
        Self::Save(SaveError::missing_endpoint(endpoint))
    }

    /// 루트나 `http`·`sampling` 같은 칸의 필수 값 없음·타입 오류 반환.
    pub(super) fn invalid_field(field: impl Into<String>) -> Self {
        Self::Validation(ValidationError::invalid_field(field))
    }

    /// 엔드포인트 항목 칸의 필수 값 없음·타입 오류 반환.
    pub(super) fn invalid_endpoint_field(endpoint: &str, field: impl Into<String>) -> Self {
        Self::Validation(ValidationError::invalid_endpoint_field(endpoint, field))
    }

    /// 모델 항목 칸의 필수 값 없음·타입 오류 반환.
    pub(super) fn invalid_model_field(
        endpoint: &str,
        model: &str,
        field: impl Into<String>,
    ) -> Self {
        Self::Validation(ValidationError::invalid_model_field(endpoint, model, field))
    }

    /// 모델 id trim 후 비우기 오류 반환. `model`은 trim한 모델 이름.
    pub(super) fn empty_model_id(endpoint: &str, model: &str) -> Self {
        Self::Validation(ValidationError::empty_model_id(endpoint, model))
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 함.
        write!(f, "ConfigError({})", self)
    }
}
