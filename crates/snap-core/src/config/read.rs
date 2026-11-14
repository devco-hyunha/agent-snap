//! 설정 읽기·해석·검증 무: 파일을 값으로 읽고 규칙을 확인합니다.

mod agent;
mod context_window;
mod extra;
pub(crate) mod extra_json;
mod http;
mod keys;
mod model;
mod parse;
mod reasoning;
mod sampling;
mod selection;
mod target_language;
mod tool_call_form;
pub(crate) mod types;
pub(crate) mod validate;

pub use selection::SelectionError;
pub use types::{
    AgentSettings, Config, EndpointConfig, EndpointKind, ExtraValue, HttpSettings, ModelConfig,
    Reasoning, Sampling, ToolCallForm,
};

/// 03-2 에이전트가 설정 검증 규칙을 그대로 재사용하게 여는 자리입니다(D15, DRY).
pub(crate) use validate::{RESERVED_EXTRA_KEYS, is_valid_temperature};
