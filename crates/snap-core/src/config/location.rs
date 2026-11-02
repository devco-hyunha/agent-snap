//! 사용자 설정 폴더 위치 계산입니다. 환경 정보를 주입받아 계산만 하고 파일은 건드리지 않습니다.

use std::path::{Path, PathBuf};

use super::error::ConfigError;

/// 설정 폴더 이름입니다.
const APP_DIR_NAME: &str = "agent-snap";

/// 운영체제 관례를 가르는 종류입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostOs {
    /// Windows. Roaming AppData 아래에 둡니다.
    Windows,
    /// 그 밖의 운영체제. XDG 관례를 따릅니다.
    Other,
}

/// 설정 폴더 계산에 필요한 환경 정보입니다. 호출한 쪽이 실제 환경에서 읽어 넣습니다.
#[derive(Debug, Clone)]
pub struct ConfigEnv {
    /// 운영체제 종류입니다.
    pub os: HostOs,
    /// 환경 변수 `AGENT_SNAP_CONFIG_DIR` 값입니다.
    pub config_dir_override: Option<PathBuf>,
    /// Windows의 Roaming AppData 폴더(`APPDATA`)입니다.
    pub appdata: Option<PathBuf>,
    /// `XDG_CONFIG_HOME` 값입니다.
    pub xdg_config_home: Option<PathBuf>,
    /// 홈 폴더입니다. `XDG_CONFIG_HOME`이 없을 때 `.config` 아래를 씁니다.
    pub home: Option<PathBuf>,
}

/// 사용자 설정 폴더를 정합니다(DECISIONS D19).
///
/// `AGENT_SNAP_CONFIG_DIR`에 값이 있으면 그 폴더를 쓰고, 없으면 Windows는 Roaming AppData
/// 아래 `agent-snap`, 그 밖은 `$XDG_CONFIG_HOME/agent-snap`(없으면 `~/.config/agent-snap`)입니다.
///
/// # 오류
/// 관례 폴더를 계산할 환경 정보가 없으면 [`ConfigError::ConfigDirUnknown`]입니다.
pub fn resolve_config_dir(env: &ConfigEnv) -> Result<PathBuf, ConfigError> {
    if let Some(dir) = non_empty(&env.config_dir_override) {
        return Ok(dir.to_path_buf());
    }
    let convention_dir = match env.os {
        HostOs::Windows => non_empty(&env.appdata).map(|appdata| appdata.join(APP_DIR_NAME)),
        HostOs::Other => non_empty(&env.xdg_config_home)
            .map(|xdg| xdg.join(APP_DIR_NAME))
            .or_else(|| non_empty(&env.home).map(|home| home.join(".config").join(APP_DIR_NAME))),
    };
    convention_dir.ok_or(ConfigError::ConfigDirUnknown)
}

fn non_empty(path: &Option<PathBuf>) -> Option<&Path> {
    path.as_deref().filter(|path| !path.as_os_str().is_empty())
}
