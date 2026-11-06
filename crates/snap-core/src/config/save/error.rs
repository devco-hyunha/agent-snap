//! 저장 오류가 담는 것을 정하는 타입입니다(02-8 spec의 '오류 종류와 담는 것' 표 2번째 줄).
//!
//! [`ConfigError::SaveIo`]는 표 2번째 줄대로 대상 파일 종류(설정, 자격)와 오류 종류만
//! Display·Debug에 남깁니다. 담지 않는 것(경로 전체, 키 값, base_url)은 표시에 나올 자리가
//! 없습니다. 검증 실패(표 1번째 줄)는 02-2와 같은 [`ConfigError`] 변형을 저장이 그대로
//! 넘기는 것이라 새 타입이 필요 없습니다. 표 3번째 줄은 [`ConfigError::MissingEndpoint`]이
//! 엔드포인트 이름만 담는 자리입니다.
//! Secret 래퍼 타입은 담는 자리에 키 값이 없어서 쓰지 않습니다(작업 5 범위).
//!
//! 저장이 파일 이름과 원인을 대상 파일 종류와 오류 종류로 바꾸는 자리는 이 파일의
//! [`save_io_target`]과 [`save_io_kind`] 두 함수입니다(dev-principles 3절 DRY: 같은 분류를
//! 여러 파일에 다시 적지 않습니다).

use std::fmt;
use std::io::ErrorKind;

use super::super::load::CONFIG_FILE_NAME;

pub(super) const TEMP_WRITE: SaveIoKind = SaveIoKind::TempWrite;
pub(super) const REPLACE: SaveIoKind = SaveIoKind::Replace;
pub(super) const READ: SaveIoKind = SaveIoKind::Read;

/// 저장이 파일 쓰기에 실패한 오류를 만드는 자리입니다. 저장이 파일 종류와 오류 종류만
/// 정해서 넘깁니다(spec 표 2번째 줄).
pub fn save_io(file: &'static str, kind: SaveIoKind) -> super::super::error::ConfigError {
    super::super::error::SaveError::SaveIo {
        target: save_io_target(file),
        kind,
    }
    .into()
}

/// 저장이 파일 쓰기에 실패했을 때 밝히는 대상 파일 종류입니다. 파일 이름과 경로가 아니라
/// 종류만 밝힙니다(spec 표 2번째 줄 '담는 것: 대상 파일 종류').
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveIoTarget {
    /// 설정 파일(`agent-snap.jsonc`)을 고치다가 실패했습니다.
    Config,
    /// 자격 파일(`credentials.jsonc`)을 고치다가 실패했습니다.
    Credentials,
}

impl fmt::Display for SaveIoTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Config => "설정",
            Self::Credentials => "자격",
        };
        write!(f, "{text} 파일")
    }
}

/// 저장이 파일 쓰기에 실패했을 때의 오류 종류입니다. 사람이 판단할 수 있는 종류만 밝힙니다.
/// `std::io::Error`가 밝히는 파일 경로는 밝히지 않습니다(spec 표 2번째 줄 '담지 않는 것:
/// 경로 전체').
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveIoKind {
    /// 임시 파일에 쓰는 중에 실패했습니다.
    TempWrite,
    /// 기존 파일을 교체하는 중에 실패했습니다.
    Replace,
    /// 파일을 읽는 중에 실패했습니다.
    Read,
    /// 쓰기 권한이 없습니다.
    PermissionDenied,
    /// 파일·폴더가 없습니다.
    NotFound,
    /// 같은 이름의 파일이 이미 있습니다.
    AlreadyExists,
    /// 파일이 다른 프로세스에 열려 있어 교체를 못 했습니다(Windows 공유 위반).
    SharingViolation,
    /// 그 밖의 실패입니다.
    Other,
}

impl fmt::Display for SaveIoKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::TempWrite => "임시 파일 쓰기",
            Self::Replace => "교체",
            Self::Read => "읽기",
            Self::PermissionDenied => "권한",
            Self::NotFound => "대상 없음",
            Self::AlreadyExists => "이미 있음",
            Self::SharingViolation => "사용 중",
            Self::Other => "알 수 없음",
        };
        write!(f, "{text} 실패")
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

/// 저장이 파일 이름 상수를 대상 파일 종류로 바꾸는 자리입니다. 저장이 파일 종류를 다시
/// 적지 않게 이 함수 하나로 모습니다.
pub fn save_io_target(file: &'static str) -> SaveIoTarget {
    if file == CONFIG_FILE_NAME {
        SaveIoTarget::Config
    } else {
        SaveIoTarget::Credentials
    }
}

/// Windows 공유 위반 코드입니다. Windows가 아닌 곳에서는 0이고, 원인의 번호가 0인 실패는
/// 이 종류로 잡히지 않습니다(std가 원인을 만들 때 0번을 쓰지 않습니다).
const SHARING_VIOLATION: i32 = 32;
