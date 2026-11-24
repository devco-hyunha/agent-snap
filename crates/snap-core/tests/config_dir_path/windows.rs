//! Windows 설정 폴더 경로 시험.
use std::path::PathBuf;

use snap_core::config::config_dir;

#[test]
fn windows_config_dir_under_appdata() {
    let app_data = std::env::var_os("APPDATA").expect("test reads APPDATA");
    let expected = PathBuf::from(&app_data).join("snap");

    let got = config_dir();

    assert_eq!(got, expected);
}
