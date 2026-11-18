//! 설정 파일(`agent-snap.jsonc`)을 읽어 검증하고, 엔드포인트 키를 자격 파일에서 정하는 모듈.

mod error;
mod load;
mod location;
mod read;
mod save;

pub use error::{ConfigError, ReadingError, SaveError, ValidationError};
pub use error::{EndpointIssue, FieldIssue, ModelIssue, SectionIssue};
pub use load::{CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, load_config, load_credentials};
pub use location::{ConfigEnv, HostOs, resolve_config_dir};
pub use read::{
    AgentSettings, Config, EndpointConfig, EndpointKind, ExtraValue, HttpSettings, ModelConfig,
    Reasoning, Sampling, SelectionError, ToolCallForm,
};
pub use save::{
    EndpointSave, KeyRename, ModelSave, SaveIoKind, SaveIoTarget, remove_endpoint, remove_key,
    rename_endpoint, rename_key, save_endpoint, save_io, save_io_target, save_key,
};

pub(crate) use read::extra_json;
pub(crate) use read::{RESERVED_EXTRA_KEYS, is_valid_temperature};

#[cfg(test)]
mod tests {
    mod agent;
    mod extra_json;
    mod keys;
    mod location;
    mod reading;
    mod redaction;
    mod save;
    mod selection;
    mod support;
    mod validation;
}
