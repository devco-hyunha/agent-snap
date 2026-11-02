//! 설정 폴더 위치 테스트(명세 AC 1): 환경 정보를 주입해 우선순위와 운영체제 관례를 확인한다.

use std::path::PathBuf;

use super::support::{TempDir, empty_env};
use crate::config::{
    CONFIG_FILE_NAME, ConfigEnv, ConfigError, HostOs, load_config, resolve_config_dir,
};

/// AC 1: `AGENT_SNAP_CONFIG_DIR` 값이 있으면 운영체제와 관계없이 그 폴더를 쓴다.
#[test]
fn override_dir_wins_over_os_convention() {
    for os in [HostOs::Windows, HostOs::Other] {
        let env = ConfigEnv {
            config_dir_override: Some(PathBuf::from("custom-dir")),
            appdata: Some(PathBuf::from("appdata")),
            xdg_config_home: Some(PathBuf::from("xdg")),
            home: Some(PathBuf::from("home")),
            ..empty_env(os)
        };

        assert_eq!(
            resolve_config_dir(&env).expect("폴더"),
            PathBuf::from("custom-dir")
        );
    }
}

/// AC 1: 값이 비어 있는 `AGENT_SNAP_CONFIG_DIR`은 없는 것으로 보고 관례를 따른다.
#[test]
fn empty_override_falls_back_to_convention() {
    let env = ConfigEnv {
        config_dir_override: Some(PathBuf::new()),
        appdata: Some(PathBuf::from("appdata")),
        ..empty_env(HostOs::Windows)
    };

    assert_eq!(
        resolve_config_dir(&env).expect("폴더"),
        PathBuf::from("appdata").join("agent-snap")
    );
}

/// AC 1: Windows는 Roaming AppData 아래 `agent-snap`이다.
#[test]
fn windows_uses_roaming_appdata() {
    let env = ConfigEnv {
        appdata: Some(PathBuf::from("roaming")),
        // Windows에서는 XDG 값이 있어도 쓰지 않는다.
        xdg_config_home: Some(PathBuf::from("xdg")),
        ..empty_env(HostOs::Windows)
    };

    assert_eq!(
        resolve_config_dir(&env).expect("폴더"),
        PathBuf::from("roaming").join("agent-snap")
    );
}

/// AC 1: 그 밖의 운영체제는 `XDG_CONFIG_HOME`이 있으면 그 아래를 쓴다.
#[test]
fn other_os_uses_xdg_config_home_first() {
    let env = ConfigEnv {
        xdg_config_home: Some(PathBuf::from("xdg")),
        home: Some(PathBuf::from("home")),
        ..empty_env(HostOs::Other)
    };

    assert_eq!(
        resolve_config_dir(&env).expect("폴더"),
        PathBuf::from("xdg").join("agent-snap")
    );
}

/// AC 1: `XDG_CONFIG_HOME`이 없으면 홈 아래 `.config/agent-snap`이다.
#[test]
fn other_os_falls_back_to_home_dot_config() {
    let env = ConfigEnv {
        home: Some(PathBuf::from("home")),
        ..empty_env(HostOs::Other)
    };

    assert_eq!(
        resolve_config_dir(&env).expect("폴더"),
        PathBuf::from("home").join(".config").join("agent-snap")
    );
}

/// AC 1: 관례 폴더를 계산할 정보가 없으면 경로 없는 오류다.
#[test]
fn missing_environment_information_is_an_error() {
    for os in [HostOs::Windows, HostOs::Other] {
        let error = resolve_config_dir(&empty_env(os)).expect_err("정보가 없으면 오류여야 합니다");

        assert!(matches!(error, ConfigError::ConfigDirUnknown));
    }
}

/// AC 1: 지정한 폴더(임시 폴더)를 계산해 그 안의 설정을 읽는다. 위치 계산과 읽기가 독립이다.
#[test]
fn resolved_override_dir_is_used_to_read_config() {
    let folder = TempDir::new("override");
    folder.write(CONFIG_FILE_NAME, r#"{"target_language":"ja"}"#);
    let env = ConfigEnv {
        config_dir_override: Some(folder.path().to_path_buf()),
        ..empty_env(HostOs::Other)
    };

    let dir = resolve_config_dir(&env).expect("폴더");
    let config = load_config(&dir).expect("설정을 읽어야 합니다");

    assert_eq!(config.target_language, "ja");
}
