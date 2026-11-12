//! 칸 형식 규칙(필수 칸·타형)에 걸린 검증 실패만 담는 부분 열거형입니다(감싸기는 `../validation.rs`).
//!
//! 칸 이름과 형식을 밝히는 무리입니다. 읽기 코드가 새 칸을 추가할 때만 함께 바뀌므로 그
//! 변경 이유만 맡습니다(dev-principles 2절 SRP). Display·Debug에 입력 값은 담지 않고,
//! 표시 문구는 이전 검증 오류와 한 글자도 같습니다.

use std::fmt;

#[derive(thiserror::Error)]
pub enum FieldIssue {
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

impl fmt::Debug for FieldIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 합니다.
        write!(f, "FieldIssue({})", self)
    }
}
