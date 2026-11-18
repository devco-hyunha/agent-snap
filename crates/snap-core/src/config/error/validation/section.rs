//! 섹션 키 규칙(`sampling`·`http`·`agent`의 모르는 항목과 백오프 범위)에 걸린 검증 실패만 담는 부분 열거형.
//!
//! Display·Debug에 입력 값과 키 값을 담지 않는 제약.

use std::fmt;

#[derive(thiserror::Error)]
pub enum SectionIssue {
    /// 샘플링 항목이 범위를 벗어난 상태. 값은 담지 않음.
    #[error("설정 오류: 샘플링 `{item}`이(가) 모델 `{model}`에서 범위를 벗어났습니다")]
    SamplingOutOfRange { item: String, model: String },
    /// 지원하지 않는 샘플링 항목.
    #[error("설정 오류: 지원하지 않는 샘플링 항목 `{item}`")]
    UnknownSamplingItem { item: String },
    /// `http`에 인정하지 않는 키가 있는 상태.
    #[error("설정 오류: 지원하지 않는 `http` 항목 `{item}`")]
    UnknownHttpItem { item: String },
    /// `agent` 섹션에 인정하지 않는 키가 있는 상태.
    #[error("설정 오류: 지원하지 않는 `agent` 항목 `{item}`")]
    UnknownAgentItem { item: String },
    /// `http.backoff_min`이 `http.backoff_max`보다 큰 상태. 항목 이름만 담고 값은 담지 않음.
    #[error("설정 오류: `http.backoff_min`이(가) `http.backoff_max`보다 큽니다")]
    BackoffRangeInverted,
}

impl fmt::Debug for SectionIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 함.
        write!(f, "SectionIssue({})", self)
    }
}
