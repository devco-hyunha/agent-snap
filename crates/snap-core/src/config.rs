//! 설정 파일(`agent-snap.jsonc`)을 읽어 검증하고, 엔드포인트의 키를 자격 파일에서 정하는 모듈입니다.

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

// 크레이트 안 호출부가 `crate::config::extra_json` 경로를 그대로 쓰게 부모에서 재수출합니다.
pub(crate) use read::extra_json;
// 03-2 에이전트가 설정 검증 규칙과 예약 키 목록을 그대로 재사용하게 여는 자리입니다(D15, DRY).
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
