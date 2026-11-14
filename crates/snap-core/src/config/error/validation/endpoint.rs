//! 엔드포인트 항목 규칙에 걸린 검증 실패만 담는 부분 열거형입니다(감싸기는 `../validation.rs`).
//!
//! 엔드포인트 이름·`kind`·`base_url`·키 칸과 옛 `models` 칸을 다루는 무리입니다. 엔드포인트
//! 항목 규정이 바뀔 때만 함께 바뀌므로 그 변경 이유만 맡습니다(dev-principles 2절 SRP).
//! Display·Debug에 입력 값은 담지 않고, 표시 문구는 이전 검증 오류와 한 글자도 같습니다.

use std::fmt;

#[derive(thiserror::Error)]
pub enum EndpointIssue {
    /// 최상위에 옛 `models` 칸이 있습니다.
    #[error("설정 오류: 모델은 endpoints[].models에 둡니다")]
    LegacyTopLevelModels,
    /// 엔드포인트에 키 칸(`api_key` 또는 `api_key_env`)이 있습니다. 값은 담지 않습니다.
    #[error(
        "설정 오류: 엔드포인트 `{endpoint}`에 키 칸이 있습니다. 키는 credentials.jsonc에 둡니다"
    )]
    EndpointKeyField { endpoint: String },
    /// 엔드포인트 이름이 중복됩니다(trim 후 정확히 같은 문자열).
    #[error("설정 오류: 엔드포인트 이름 `{endpoint}`이(가) 중복됩니다")]
    DuplicateEndpoint { endpoint: String },
    /// 엔드포인트 이름이 trim 후 비어 있습니다.
    #[error("설정 오류: 엔드포인트 이름이 비어 있습니다")]
    EmptyEndpointName,
    /// 엔드포인트 이름에 `/`가 있습니다.
    #[error("설정 오류: 엔드포인트 이름 `{endpoint}`에 `/`를 넣을 수 없습니다")]
    EndpointNameHasSlash { endpoint: String },
    /// `kind` 값이 지원 목록에 없습니다. 입력 값은 담지 않습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `kind` 값이 올바르지 않습니다")]
    InvalidKind { endpoint: String },
    /// `base_url` 값이 검증을 통과하지 못했습니다. 입력 값은 담지 않습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `base_url` 값이 올바르지 않습니다")]
    InvalidBaseUrl { endpoint: String },
}

impl fmt::Debug for EndpointIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 합니다.
        write!(f, "EndpointIssue({})", self)
    }
}
