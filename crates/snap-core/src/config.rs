//! 설정 파일(`agent-snap.jsonc`)을 읽어 검증하고, 엔드포인트의 키를 자격 파일에서 정하는 모듈입니다.

mod error;
mod http;
mod keys;
mod load;
mod location;
mod parse;
mod sampling;
mod types;
mod validate;

pub use error::ConfigError;
pub use load::{CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, load_config, load_credentials};
pub use location::{ConfigEnv, HostOs, resolve_config_dir};
pub use types::{
    Config, EndpointConfig, EndpointKind, ExtraValue, HttpSettings, ModelConfig, Reasoning,
    Sampling,
};

#[cfg(test)]
mod tests {
    mod keys;
    mod location;
    mod reading;
    mod redaction;
    mod support;
    mod validation;
}
