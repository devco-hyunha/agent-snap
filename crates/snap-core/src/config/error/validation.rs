//! 설정 파일 항목을 검증해 채운 값(엔드포인트·모델 이름과 칸, `http`·`sampling` 칸)이
//! 규칙에 맞지 않을 때의 실패 이유만 담는 열거형입니다.

use std::fmt;

/// 설정 파일의 항목과 칸이 02-2 검증 규칙을 통과하지 못했을 때의 오류입니다.
///
/// 검증 규칙이 바뀌는 이유로만 바뀌며, 읽기·저장 이유는 각각 `error/reading.rs`·
/// `error/save.rs`가 맡습니다(명세 AC 2·4·5). Display·Debug에 입력 값(base_url 호스트·
/// 키 값)을 담지 않습니다.
#[derive(thiserror::Error)]
pub enum ValidationError {
    /// 최상위에 옛 `models` 칸이 있습니다.
    #[error("설정 오류: 모델은 endpoints[].models에 둡니다")]
    LegacyTopLevelModels,
    /// 엔드포인트에 키 칸(`api_key` 또는 `api_key_env`)이 있습니다. 값은 담지 않습니다.
    #[error(
        "설정 오류: 엔드포인트 `{endpoint}`에 키 칸이 있습니다. 키는 credentials.jsonc에 둡니다"
    )]
    EndpointKeyField { endpoint: String },
    /// 모델에 `endpoint` 칸이 있습니다.
    #[error("설정 오류: 모델 `{model}`에 `endpoint` 칸이 있습니다")]
    ModelEndpointField { model: String },
    /// 엔드포인트 이름이 중복됩니다(trim 후 정확히 같은 문자열).
    #[error("설정 오류: 엔드포인트 이름 `{endpoint}`이(가) 중복됩니다")]
    DuplicateEndpoint { endpoint: String },
    /// 같은 엔드포인트 안에서 모델 이름이 중복됩니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 이름 `{model}`이(가) 중복됩니다")]
    DuplicateModel { endpoint: String, model: String },
    /// 엔드포인트 이름이 trim 후 비어 있습니다.
    #[error("설정 오류: 엔드포인트 이름이 비어 있습니다")]
    EmptyEndpointName,
    /// 모델 이름이 trim 후 비어 있습니다. 이름이 비어 모델 이름은 담지 못합니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 이름이 비어 있습니다")]
    EmptyModelName { endpoint: String },
    /// 모델 id가 trim 후 비어 있습니다. trim한 모델 이름을 담습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 `{model}`의 id가 비어 있습니다")]
    EmptyModelId { endpoint: String, model: String },
    /// 엔드포인트 이름에 `/`가 있습니다.
    #[error("설정 오류: 엔드포인트 이름 `{endpoint}`에 `/`를 넣을 수 없습니다")]
    EndpointNameHasSlash { endpoint: String },
    /// 모델 이름에 `/`가 있습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 이름 `{model}`에 `/`를 넣을 수 없습니다")]
    ModelNameHasSlash { endpoint: String, model: String },
    /// `kind` 값이 지원 목록에 없습니다. 입력 값은 담지 않습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `kind` 값이 올바르지 않습니다")]
    InvalidKind { endpoint: String },
    /// `base_url` 값이 검증을 통과하지 못했습니다. 입력 값은 담지 않습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `base_url` 값이 올바르지 않습니다")]
    InvalidBaseUrl { endpoint: String },
    /// `reasoning` 값이 지원 목록에 없습니다. 입력 값은 담지 않습니다.
    #[error("설정 오류: 모델 `{model}`의 `reasoning` 값이 올바르지 않습니다")]
    InvalidReasoning { model: String },
    /// `extra`에 클라이언트가 채우는 예약 키가 있습니다.
    #[error("설정 오류: 모델 `{model}`의 `extra`에 예약 키 `{key}`가 있습니다")]
    ExtraReservedKey { model: String, key: String },
    /// 샘플링 항목이 범위를 벗어났습니다. 값은 담지 않습니다.
    #[error("설정 오류: 샘플링 `{item}`이(가) 모델 `{model}`에서 범위를 벗어났습니다")]
    SamplingOutOfRange { item: String, model: String },
    /// 지원하지 않는 샘플링 항목입니다.
    #[error("설정 오류: 지원하지 않는 샘플링 항목 `{item}`")]
    UnknownSamplingItem { item: String },
    /// `http`에 인정하지 않는 키가 있습니다.
    #[error("설정 오류: 지원하지 않는 `http` 항목 `{item}`")]
    UnknownHttpItem { item: String },
    /// `http.backoff_min`이 `http.backoff_max`보다 큽니다. 항목 이름만 담고 값은 담지 않습니다(D21).
    #[error("설정 오류: `http.backoff_min`이(가) `http.backoff_max`보다 큽니다")]
    BackoffRangeInverted,
    /// 필수 칸이 없거나 타입이 맞지 않습니다. 칸 이름만 담습니다.
    #[error("설정 오류: `{field}` 칸이 없거나 형식이 올바르지 않습니다")]
    InvalidField { field: String },
    /// 엔드포인트 항목의 필수 칸이 없거나 타입이 맞지 않습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `{field}` 칸이 없거나 형식이 올바르지 않습니다")]
    InvalidEndpointField { endpoint: String, field: String },
    /// 모델 항목의 필수 칸이 없거나 타입이 맞지 않습니다.
    #[error(
        "설정 오류: 엔드포인트 `{endpoint}`의 모델 `{model}`의 `{field}` 칸이 없거나 형식이 올바르지 않습니다"
    )]
    InvalidModelField {
        endpoint: String,
        model: String,
        field: String,
    },
}

impl ValidationError {
    /// 루트나 `http`·`sampling` 같은 칸의 필수 값이 없거나 타입이 맞지 않을 때의 오류입니다.
    pub(in crate::config) fn invalid_field(field: impl Into<String>) -> Self {
        Self::InvalidField {
            field: field.into(),
        }
    }

    /// 엔드포인트 항목의 칸이 없거나 타입이 맞지 않을 때의 오류입니다.
    pub(crate) fn invalid_endpoint_field(endpoint: &str, field: impl Into<String>) -> Self {
        Self::InvalidEndpointField {
            endpoint: endpoint.to_owned(),
            field: field.into(),
        }
    }

    /// 모델 항목의 칸이 없거나 타입이 맞지 않을 때의 오류입니다.
    pub(in crate::config) fn invalid_model_field(
        endpoint: &str,
        model: &str,
        field: impl Into<String>,
    ) -> Self {
        Self::InvalidModelField {
            endpoint: endpoint.to_owned(),
            model: model.to_owned(),
            field: field.into(),
        }
    }

    /// 모델 이름이 trim 후 비었을 때의 오류입니다.
    pub(in crate::config) fn empty_model_name(endpoint: &str) -> Self {
        Self::EmptyModelName {
            endpoint: endpoint.to_owned(),
        }
    }

    /// 모델 id가 trim 후 비었을 때의 오류입니다. `model`은 trim한 모델 이름입니다.
    pub(in crate::config) fn empty_model_id(endpoint: &str, model: &str) -> Self {
        Self::EmptyModelId {
            endpoint: endpoint.to_owned(),
            model: model.to_owned(),
        }
    }
}

impl fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 합니다.
        write!(f, "ValidationError({})", self)
    }
}
