//! 설정 값 타입을 주제별로 나눈 파일들을 한 자리에서 모으고, 기본값 상수를 둡니다.
//!
//! 값은 읽는 시점에 검증되므로 이후 단계는 그대로 믿고 씁니다.

pub mod forms;
pub mod http;
pub mod model;
pub mod sampling;

pub use forms::{EndpointKind, Reasoning, ToolCallForm};
pub use http::HttpSettings;
pub use model::{Config, EndpointConfig, ModelConfig};
pub use sampling::{ExtraValue, Sampling};

/// `target_language`를 생략했을 때의 기본값입니다.
pub(in crate::config) const DEFAULT_TARGET_LANGUAGE: &str = "ko";

/// `context_window`를 생략했을 때의 기본값입니다(D29). 과대 기본을 막으려 흔한 로컬 창인 64K를 씁니다.
pub(in crate::config) const DEFAULT_CONTEXT_WINDOW: u64 = 65536;

/// `tool_call_form`을 생략했을 때의 기본값입니다(D50 결정 7). 1안 OpenAI 경로를 그대로 선언합니다.
pub(in crate::config) const DEFAULT_TOOL_CALL_FORM: ToolCallForm = ToolCallForm::OpenAi;
