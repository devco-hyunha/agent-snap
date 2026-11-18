//! 엔드포인트 항목 규칙에 걸린 검증 실패만 담는 부분 열거형.
//!
//! Display·Debug에 입력 값과 키 값을 담지 않는 제약.

use std::fmt;

#[derive(thiserror::Error)]
pub enum EndpointIssue {
    /// 최상위에 옛 `models` 칸이 있는 상태.
    #[error("설정 오류: 모델은 endpoints[].models에 둡니다")]
    LegacyTopLevelModels,
    /// 엔드포인트에 키 칸(`api_key` 또는 `api_key_env`)이 있는 상태. 값은 담지 않음.
    #[error(
        "설정 오류: 엔드포인트 `{endpoint}`에 키 칸이 있습니다. 키는 credentials.jsonc에 둡니다"
    )]
    EndpointKeyField { endpoint: String },
    /// 엔드포인트 이름이 중복된 상태(trim 후 정확히 같은 문자열).
    #[error("설정 오류: 엔드포인트 이름 `{endpoint}`이(가) 중복됩니다")]
    DuplicateEndpoint { endpoint: String },
    /// 엔드포인트 이름이 trim 후 빈 상태.
    #[error("설정 오류: 엔드포인트 이름이 비어 있습니다")]
    EmptyEndpointName,
    /// 엔드포인트 이름에 `/`가 있는 상태.
    #[error("설정 오류: 엔드포인트 이름 `{endpoint}`에 `/`를 넣을 수 없습니다")]
    EndpointNameHasSlash { endpoint: String },
    /// `kind` 값이 지원 목록에 없는 상태. 입력 값은 담지 않음.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `kind` 값이 올바르지 않습니다")]
    InvalidKind { endpoint: String },
    /// `base_url` 값이 검증을 통과하지 못한 상태. 입력 값은 담지 않음.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `base_url` 값이 올바르지 않습니다")]
    InvalidBaseUrl { endpoint: String },
}

impl fmt::Debug for EndpointIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 함.
        write!(f, "EndpointIssue({})", self)
    }
}
