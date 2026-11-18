//! 저장이 실패 지점을 오류로 조립하는 자리

use std::io::ErrorKind;

use super::super::load::CONFIG_FILE_NAME;
use super::error::{SaveIoKind, SaveIoTarget};

/// 저장이 파일 쓰기에 실패한 오류를 만듦. 파일 종류와 오류 종류만 정해서 넘김
pub fn save_io(file: &'static str, kind: SaveIoKind) -> super::super::error::ConfigError {
    super::super::error::SaveError::SaveIo {
        target: save_io_target(file),
        kind,
    }
    .into()
}

/// 파일 이름 상수를 대상 파일 종류로 변환. 파일 종류를 다시 적지 않게 이 함수 하나로 모음
pub fn save_io_target(file: &'static str) -> SaveIoTarget {
    if file == CONFIG_FILE_NAME {
        SaveIoTarget::Config
    } else {
        SaveIoTarget::Credentials
    }
}

/// 원인을 저장 오류 종류로 정함. 지점 이름(`TEMP_WRITE`·`REPLACE`·`READ`)에 그 밖의 실패를 더함. Windows 공유 위반처럼 사람이 판단할 수 있는 원인은 지점 이름보다 원인 번호를 우선으로 봄
pub(super) fn save_io_kind(zero: SaveIoKind, source: &std::io::Error) -> SaveIoKind {
    // Windows 공유 위반은 std가 종류로 나누지 않아 번호로만 봄. 열려 있는 파일을
    // 교체할 때 나는 번호는 Windows 실측으로 확인한 값
    if source.raw_os_error() == Some(SHARING_VIOLATION) {
        return SaveIoKind::SharingViolation;
    }
    match source.kind() {
        ErrorKind::PermissionDenied => SaveIoKind::PermissionDenied,
        ErrorKind::NotFound => SaveIoKind::NotFound,
        ErrorKind::AlreadyExists => SaveIoKind::AlreadyExists,
        _ => zero,
    }
}

/// Windows 공유 위반 코드. Windows가 아닌 곳에서는 0이고, 원인의 번호가 0인 실패는 이 종류로 잡히지 않음(std가 원인을 만들 때 0번을 쓰지 않음)
const SHARING_VIOLATION: i32 = 32;
