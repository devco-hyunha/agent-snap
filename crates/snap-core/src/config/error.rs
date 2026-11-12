//! 설정 읽기·검증·저장 오류의 합쳐진 타입: 세 부분 열거형을 감싸기만 합니다.
//!
//! 부분 열거형의 변형과 만들기 도움 함수는 아래 `pub use`로 그대로 재수출합니다.
//! 감싸는 열거형의 변형 이름은 호출부에서 `ConfigError` 경로를 그대로 쓰는 기준으로
//! 부분 열거형 타입 이름을 붙인 생성 함수를 씁니다.

mod reading;
mod save;
mod validation;

pub use reading::ReadingError;
pub use save::SaveError;
pub use validation::{EndpointIssue, FieldIssue, ModelIssue, SectionIssue, ValidationError};

use std::fmt;

use crate::credentials::CredentialsError;
use crate::jsonc::JsoncError;

/// 설정 파일 읽기·검증·저장에 실패했을 때 나오는 오류입니다.
///
/// 변경 이유가 서로 다른 셋(읽기·검증·저장)을 각각의 부분 열거형으로 놓고, 이 타입은 그
/// 셋을 감싸는 자리만 맡습니다(하네스 1절 SRP). 표시 규칙은 부분 열거형마다 있어서,
/// 합쳐서 드러나는 문구는 이전과 같습니다(명세 AC 2·4·5).
#[derive(thiserror::Error)]
pub enum ConfigError {
    /// 파일을 찾고 읽고 해석하는 단계가 실패했습니다.
    #[error(transparent)]
    Reading(#[from] ReadingError),
    /// 채운 값이 02-2 검증 규칙을 통과하지 못했습니다.
    #[error(transparent)]
    Validation(#[from] ValidationError),
    /// 파일을 쓰다가 실패했습니다.
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
    // 이 도움 함수들은 부분 열거형의 같은 생성 함수를 공개 경로(`ConfigError`) 이름으로
    // 이어 주는 forwarding입니다. 새 도움 생성 함수는 부분 열거형(`ReadingError`·
    // `ValidationError`·`SaveError`)의 대응 자리에 먼저 만들고, 호출부가 공개 경로를 써야
    // 할 때만 이 타입에 forwarding을 더합니다.
    //
    // 저장이 바꿀 엔드포인트가 없을 때의 오류입니다. spec 표 3번째 줄대로 이름만 담습니다.
    pub(super) fn missing_endpoint(endpoint: &str) -> Self {
        Self::Save(SaveError::missing_endpoint(endpoint))
    }

    /// 루트나 `http`·`sampling` 같은 칸의 필수 값이 없거나 타입이 맞지 않을 때의 오류입니다.
    pub(super) fn invalid_field(field: impl Into<String>) -> Self {
        Self::Validation(ValidationError::invalid_field(field))
    }

    /// 엔드포인트 항목의 칸이 없거나 타입이 맞지 않을 때의 오류입니다.
    pub(super) fn invalid_endpoint_field(endpoint: &str, field: impl Into<String>) -> Self {
        Self::Validation(ValidationError::invalid_endpoint_field(endpoint, field))
    }

    /// 모델 항목의 칸이 없거나 타입이 맞지 않을 때의 오류입니다.
    pub(super) fn invalid_model_field(
        endpoint: &str,
        model: &str,
        field: impl Into<String>,
    ) -> Self {
        Self::Validation(ValidationError::invalid_model_field(endpoint, model, field))
    }

    /// 모델 id가 trim 후 비었을 때의 오류입니다. `model`은 trim한 모델 이름입니다.
    pub(super) fn empty_model_id(endpoint: &str, model: &str) -> Self {
        Self::Validation(ValidationError::empty_model_id(endpoint, model))
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 합니다.
        write!(f, "ConfigError({})", self)
    }
}
