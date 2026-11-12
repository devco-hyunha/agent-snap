//! 모델 항목 규칙에 걸린 검증 실패만 담는 부분 열거형입니다(감싸기는 `../validation.rs`).
//!
//! 모델 이름·id·`endpoint` 칸·`reasoning`·`context_window`·`tool_call_form`·`extra`를 다루는
//! 무리입니다. 모델 항목 규정이 바뀔 때만 함께 바꾸므로 그 변경 이유만 맡습니다
//! (dev-principles 2절 SRP). Display·Debug에 입력 값은 담지 않고, 표시 문구는 이전 검증
//! 오류와 한 글자도 같습니다.

use std::fmt;

#[derive(thiserror::Error)]
pub enum ModelIssue {
    /// 모델에 `endpoint` 칸이 있습니다.
    #[error("설정 오류: 모델 `{model}`에 `endpoint` 칸이 있습니다")]
    ModelEndpointField { model: String },
    /// 같은 엔드포인트 안에서 모델 이름이 중복됩니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 이름 `{model}`이(가) 중복됩니다")]
    DuplicateModel { endpoint: String, model: String },
    /// 모델 이름이 trim 후 비어 있습니다. 이름이 비어 모델 이름은 담지 못합니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 이름이 비어 있습니다")]
    EmptyModelName { endpoint: String },
    /// 모델 id가 trim 후 비어 있습니다. trim한 모델 이름을 담습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 `{model}`의 id가 비어 있습니다")]
    EmptyModelId { endpoint: String, model: String },
    /// 모델 이름에 `/`가 있습니다.
    #[error("설정 오류: 엔드포인트 `{endpoint}`의 모델 이름 `{model}`에 `/`를 넣을 수 없습니다")]
    ModelNameHasSlash { endpoint: String, model: String },
    /// `reasoning` 값이 지원 목록에 없습니다. 입력 값은 담지 않습니다.
    #[error("설정 오류: 모델 `{model}`의 `reasoning` 값이 올바르지 않습니다")]
    InvalidReasoning { model: String },
    /// `context_window` 값이 1 이상 정수가 아닙니다. 모델 이름만 밝히고 입력 값은 담지
    /// 않습니다(D29). 값을 표시하지 않는 이유는 설정에 적힌 토큰 수가 개인 정보처럼 보일 수
    /// 있고, 오류를 화면에 그대로 보여주는 CLI에서 숫자가 번지면 혼란과 정보 노출을 막기
    /// 어렵기 때문입니다.
    #[error("설정 오류: 모델 `{model}`의 `context_window` 값이 올바르지 않습니다")]
    InvalidContextWindow { model: String },
    /// `tool_call_form` 값이 지원 목록(`openai`·`text`·`auto`)에 없습니다. 입력 값은 담지 않습니다(D50 결정 7).
    #[error("설정 오류: 모델 `{model}`의 `tool_call_form` 값이 올바르지 않습니다")]
    InvalidToolCallForm { model: String },
    /// `extra`에 클라이언트가 채우는 예약 키가 있습니다.
    #[error("설정 오류: 모델 `{model}`의 `extra`에 예약 키 `{key}`가 있습니다")]
    ExtraReservedKey { model: String, key: String },
}

impl fmt::Debug for ModelIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 합니다.
        write!(f, "ModelIssue({})", self)
    }
}
