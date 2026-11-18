//! 설정 값 타입을 주제별로 나눈 파일들을 모으고, 기본값 상수를 둠.
//!
//! 값은 읽는 시점에 검증되므로 이후 단계는 그대로 믿고 씀.

pub mod agent;
pub mod forms;
pub mod http;
pub mod model;
pub mod sampling;

pub use agent::AgentSettings;
pub use forms::{EndpointKind, Reasoning, ToolCallForm};
pub use http::HttpSettings;
pub use model::{Config, EndpointConfig, ModelConfig};
pub use sampling::{ExtraValue, Sampling};

/// `run_command_deadline` 기본값(초). 0(무한 대기)는 허용하지 않음.
pub(in crate::config) const DEFAULT_RUN_COMMAND_DEADLINE_SECONDS: u64 = 600;

/// `approval_deadline` 기본값(초). 0(무한 대기)는 허용하지 않음.
pub(in crate::config) const DEFAULT_APPROVAL_DEADLINE_SECONDS: u64 = 300;

/// `target_language` 기본값.
pub(in crate::config) const DEFAULT_TARGET_LANGUAGE: &str = "ko";

/// `context_window` 기본값. 과대 기본을 막으려 흔한 로컬 창인 64K를 씀.
pub(in crate::config) const DEFAULT_CONTEXT_WINDOW: u64 = 65536;

/// `tool_call_form` 기본값. OpenAI 경로를 그대로 선언.
pub(in crate::config) const DEFAULT_TOOL_CALL_FORM: ToolCallForm = ToolCallForm::OpenAi;
