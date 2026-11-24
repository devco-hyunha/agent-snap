//! T2 snap 설정 폴더 경로 결정 시험 묶음. 공통 시험과 OS별 시험을 각각의 파일로 둠.
#[cfg(all(unix, not(windows)))]
mod unix;
#[cfg(windows)]
mod windows;

use std::path::Path;
use std::path::PathBuf;

use snap_core::config::config_dir_in;

#[test]
fn injected_base_yields_snap_folder_under_base() {
    let base = Path::new("base-dir");

    let got = config_dir_in(base);

    assert_eq!(got, PathBuf::from("base-dir/snap"));
}
