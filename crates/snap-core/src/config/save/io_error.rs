//! 저장이 실패 지점을 오류로 조립하는 자리입니다(03-6 R3 SRP 분리).
//!
//! 표시 타입(`SaveIoTarget`·`SaveIoKind`)은 `error.rs`에 남고, 종류를 정하고 오류로 만드는
//! 조립만 이 파일이 맡습니다(dev-principles 2절 SRP). 같은 분류를 여러 파일에 다시 적지
//! 않는다는 정본은 `error.rs` 머리말에 있습니다(dev-principles 3절 DRY).

use std::io::ErrorKind;

use super::super::load::CONFIG_FILE_NAME;
use super::error::{SaveIoKind, SaveIoTarget};

/// 저장이 파일 쓰기에 실패한 오류를 만드는 자리입니다. 저장이 파일 종류와 오류 종류만
/// 정해서 넘깁니다(spec 표 2번째 줄).
pub fn save_io(file: &'static str, kind: SaveIoKind) -> super::super::error::ConfigError {
    super::super::error::SaveError::SaveIo {
        target: save_io_target(file),
        kind,
    }
    .into()
}

/// 저장이 파일 이름 상수를 대상 파일 종류로 바꾸는 자리입니다. 저장이 파일 종류를 다시
/// 적지 않게 이 함수 하나로 모습니다.
pub fn save_io_target(file: &'static str) -> SaveIoTarget {
    if file == CONFIG_FILE_NAME {
        SaveIoTarget::Config
    } else {
        SaveIoTarget::Credentials
    }
}

/// 원인을 저장 오류 종류로 정합니다. 지점 이름(`TEMP_WRITE`·`REPLACE`·`READ`)에 그 밖의
/// 실패를 더합니다. Windows 공유 위반처럼 사람이 판단할 수 있는 원인은 지점 이름보다
/// 원인 번호를 우선으로 봅니다.
pub(super) fn save_io_kind(zero: SaveIoKind, source: &std::io::Error) -> SaveIoKind {
    // Windows 공유 위반은 std가 종류로 나누지 않아 번호로만 볼 수 있습니다. 열려 있는 파일을
    // 교체할 때 나는 번호는 02-8 작업 4의 Windows 실측에서 확인했습니다(D40의 확인하지 못한 것).
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

/// Windows 공유 위반 코드입니다. Windows가 아닌 곳에서는 0이고, 원인의 번호가 0인 실패는
/// 이 종류로 잡히지 않습니다(std가 원인을 만들 때 0번을 쓰지 않습니다).
const SHARING_VIOLATION: i32 = 32;
