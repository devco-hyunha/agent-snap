//! 유닉스 설정 폴더 경로 시험.
use std::path::PathBuf;

use snap_core::config::config_dir;

#[test]
fn unix_config_dir_under_xdg_config_home() {
    let expected = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(value) if !value.is_empty() => PathBuf::from(value).join("snap"),
        _ => {
            let home = std::env::var_os("HOME").expect("test reads HOME");
            PathBuf::from(home).join(".config").join("snap")
        }
    };

    let got = config_dir();

    assert_eq!(got, expected);
}
