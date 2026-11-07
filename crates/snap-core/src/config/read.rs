//! 설정 읽기·해석·검증 무: 파일을 값으로 읽고 규칙을 확인합니다.

pub(crate) mod extra_json;
mod http;
mod keys;
mod parse;
mod sampling;
mod selection;
pub(crate) mod types;
pub(crate) mod validate;

pub use selection::SelectionError;
pub use types::{
    Config, EndpointConfig, EndpointKind, ExtraValue, HttpSettings, ModelConfig, Reasoning,
    Sampling,
};
