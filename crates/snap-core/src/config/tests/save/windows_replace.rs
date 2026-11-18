//! 안전한 쓰기 시험의 Windows 실측: Windows 파일 공유 규칙에서 교체가 실제로 실패하는지 확인
//!
//! Windows 전용 APIs를 쓰므로 Windows에서만 컴파일되는 시험 본문만 둠

use std::io::Read;
use std::os::windows::fs::OpenOptionsExt;

use super::super::support::{CONFIG_TEXT, TempDir, save_named};
use crate::config::save::atomic::replace_file;
use crate::config::save::config::save_endpoint_with;
use crate::config::{
    CONFIG_FILE_NAME, ConfigError, EndpointSave, SaveError, SaveIoKind, SaveIoTarget,
};

/// 대상 파일을 다른 열림 상태로 열어 두면 표준 교체 `std::fs::rename`이 실패하고, 기존 파일은
/// 그대로이고 임시 파일이 남지 않는지 확인
///
/// Windows 파일 공유 규칙을 실제로 확인하는 시험이라 Windows가 아닌 곳에서는 건너뜀
#[test]
fn windows_open_target_file_fails_standard_rename_and_keeps_file() {
    let folder = TempDir::new("windows-열린-파일-교체-시패");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    // 공유 설정 없는 열림이 표준 교체의 대상 열림(DELETE 요청)을 막는 Windows 조건
    let target_open = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(folder.path().join(CONFIG_FILE_NAME))
        .expect("대상 파일을 여는 것은 성공해야 합니다");
    let error = save_endpoint_with(folder.path(), &main_endpoint_save(), &replace_file)
        .expect_err("대상 파일이 공유 없이 열려 있으므로 표준 교체가 실측 실패해야 합니다");
    assert!(
        matches!(
            error,
            ConfigError::Save(SaveError::SaveIo {
                target: SaveIoTarget::Config,
                kind: SaveIoKind::SharingViolation
            })
        ),
        "설정 파일 쪽 교체 시패여야 합니다: {error}"
    );
    // 연 열림으로 읽은 내용이 원문 그대로면 기존 파일이 보존된 것
    let mut opened = target_open;
    let mut kept = String::new();
    opened
        .read_to_string(&mut kept)
        .expect("열려 있는 상태로는 읽을 수 있어야 합니다");
    assert_eq!(kept, CONFIG_TEXT, "기존 파일이 그대로여야 합니다");
    folder.assert_no_temp_files();
}

/// 읽기 전용 열림(`std::fs::File::open`) 상태에서는 표준 교체가 성공하는지 확인
///
/// 읽기 열림은 공유 위반과 무관하게 표준 교체가 성공하는지 Windows에서 실측으로 확인
#[test]
fn windows_read_only_open_allows_standard_rename() {
    let folder = TempDir::new("windows-읽기-열림-교체-성공");
    folder.write(CONFIG_FILE_NAME, CONFIG_TEXT);
    let target_open = std::fs::File::open(folder.path().join(CONFIG_FILE_NAME))
        .expect("대상 파일을 여는 것은 성공해야 합니다");
    let result = save_endpoint_with(folder.path(), &main_endpoint_save(), &replace_file);
    drop(target_open);
    result.expect("읽기 전용 열림에서는 표준 교체가 성공해야 합니다");
    assert!(
        folder
            .read(CONFIG_FILE_NAME)
            .contains("http://local.invalid/v1"),
        "교체가 반영되어 있어야 합니다"
    );
    folder.assert_no_temp_files();
}

/// 검증만 통과하면 저장할 수 있는 `main` 엔드포이트 견본
/// 교체 성공 확인용 로컬 주소를 한 자리에서만 조립
fn main_endpoint_save() -> EndpointSave {
    save_named("main", LOCAL_BASE_URL)
}

/// 교체 성공 확인용 로컬 엔드포이트 주소
const LOCAL_BASE_URL: &str = "http://local.invalid/v1";
