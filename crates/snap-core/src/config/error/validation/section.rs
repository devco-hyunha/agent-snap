//! 섹션 키 규칙(`sampling`·`http`·`agent` 섹션의 모르는 항목과 백오프 범위)에 걸린
//! 검증 실패만 담는 부분 열거형입니다(감싸기는 `../validation.rs`).
//!
//! 섹션 키 목록이 바뀔 때만 함께 바뀌므로 그 변경 이유만 맡습니다(dev-principles 2절 SRP).
//! Display·Debug에 입력 값은 담지 않고, 표시 문구는 이전 검증 오류와 한 글자도 같습니다.

use std::fmt;

#[derive(thiserror::Error)]
pub enum SectionIssue {
    /// 샘플링 항목이 범위를 벗어났습니다. 값은 담지 않습니다.
    #[error("설정 오류: 샘플링 `{item}`이(가) 모델 `{model}`에서 범위를 벗어났습니다")]
    SamplingOutOfRange { item: String, model: String },
    /// 지원하지 않는 샘플링 항목입니다.
    #[error("설정 오류: 지원하지 않는 샘플링 항목 `{item}`")]
    UnknownSamplingItem { item: String },
    /// `http`에 인정하지 않는 키가 있습니다.
    #[error("설정 오류: 지원하지 않는 `http` 항목 `{item}`")]
    UnknownHttpItem { item: String },
    /// `agent` 섹션에 인정하지 않는 키가 있습니다.
    #[error("설정 오류: 지원하지 않는 `agent` 항목 `{item}`")]
    UnknownAgentItem { item: String },
    /// `http.backoff_min`이 `http.backoff_max`보다 큽니다. 항목 이름만 담고 값은 담지 않습니다(D21).
    #[error("설정 오류: `http.backoff_min`이(가) `http.backoff_max`보다 큽니다")]
    BackoffRangeInverted,
}

impl fmt::Debug for SectionIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 합니다.
        write!(f, "SectionIssue({})", self)
    }
}
