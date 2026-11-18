//! 칸 형식 규칙(필수 칸·타형)에 걸린 검증 실패만 담는 부분 열거형.
//!
//! Display·Debug에 입력 값과 키 값을 담지 않는 제약.

use std::fmt;

#[derive(thiserror::Error)]
pub enum FieldIssue {
    /// 필수 칸이 없거나 타입이 맞지 않는 상태. 칸 이름만 담음.
    #[error("설정 오류: `{field}` 칸이 없거나 형식이 올바르지 않습니다")]
    InvalidField { field: String },
    /// 엔드포인트 항목의 필수 칸이 없거나 타입이 맞지 않는 상태.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 `{field}` 칸이 없거나 형식이 올바르지 않습니다")]
    InvalidEndpointField { endpoint: String, field: String },
    /// 모델 항목의 필수 칸이 없거나 타입이 맞지 않는 상태.
    #[error(
        "설정 오류: 엔드포인트 `{endpoint}`의 모델 `{model}`의 `{field}` 칸이 없거나 형식이 올바르지 않습니다"
    )]
    InvalidModelField {
        endpoint: String,
        model: String,
        field: String,
    },
}

impl fmt::Debug for FieldIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 함.
        write!(f, "FieldIssue({})", self)
    }
}
