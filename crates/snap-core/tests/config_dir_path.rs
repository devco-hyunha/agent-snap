//! T2 snap 설정 폴더 경로 결정 시험.
use std::path::Path;
use std::path::PathBuf;

use snap_core::config::config_dir_in;

#[test]
fn injected_base_yields_snap_folder_under_base() {
    let base = Path::new("base-dir");

    let got = config_dir_in(base);

    assert_eq!(got, PathBuf::from("base-dir/snap"));
}

#[cfg(windows)]
#[test]
fn windows_config_dir_under_appdata() {
    let app_data = std::env::var_os("APPDATA").expect("test reads APPDATA");
    let expected = PathBuf::from(&app_data).join("snap");

    let got = snap_core::config::config_dir();

    assert_eq!(got, expected);
}

#[cfg(all(unix, not(windows)))]
#[test]
fn unix_config_dir_under_xdg_config_home() {
    let expected = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(value) if !value.is_empty() => PathBuf::from(value).join("snap"),
        _ => {
            let home = std::env::var_os("HOME").expect("test reads HOME");
            PathBuf::from(home).join(".config").join("snap")
        }
    };

    let got = snap_core::config::config_dir();

    assert_eq!(got, expected);
}
