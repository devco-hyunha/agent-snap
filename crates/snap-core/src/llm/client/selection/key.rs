//! 선택 키 타입 자리. 필드와 표시 구현만 담음.

use crate::consts::limits::NAME_SEPARATOR;

/// 선택 키 `엔드포인트이름/모델이름`의 해석 결과.
///
/// 값은 trim을 지난 앞쪽·뒤쪽만 담음. 원문 문자열은 넣지 않음.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionKey {
    /// 엔드포인트 이름(trim 결과).
    pub endpoint: String,

    /// 모델 이름(trim 결과).
    pub model: String,
}

impl std::fmt::Display for SelectionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.endpoint, NAME_SEPARATOR, self.model)
    }
}
