//! 저장 오류가 담는 것을 정하는 타입.
//!
//! [`ConfigError::SaveIo`]는 대상 파일 종류(설정, 자격)와 오류 종류만 Display·Debug에 남김.
//! 담지 않는 것(경로 전체, 키 값, base_url)은 표시에 나올 자리가 없음.
//! 검증 실패는 저장이 같은 [`ConfigError`] 변형을 그대로 넘기는 것이라 새 타입이 필요 없음.
//! [`ConfigError::MissingEndpoint`]은 엔드포인트 이름만 담는 자리.
//! Secret 래퍼 타입은 담는 자리에 키 값이 없어서 쓰지 않음.
//!
//! 파일 이름과 원인을 대상 파일 종류와 오류 종류로 바꾸는 조립 자리는 `io_error.rs`의
//! `save_io_target`과 `save_io_kind` 두 함수. 같은 분류를 여러 파일에 다시 적지 않음.

use std::fmt;

pub(super) const TEMP_WRITE: SaveIoKind = SaveIoKind::TempWrite;
pub(super) const REPLACE: SaveIoKind = SaveIoKind::Replace;
pub(super) const READ: SaveIoKind = SaveIoKind::Read;

/// 저장이 파일 쓰기에 실패했을 때 밝히는 대상 파일 종류. 파일 이름과 경로가 아니라
/// 종류만 밝힘.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveIoTarget {
    /// 설정 파일(`agent-snap.jsonc`)을 고치다가 실패함.
    Config,
    /// 자격 파일(`credentials.jsonc`)을 고치다가 실패함.
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

/// 저장이 파일 쓰기에 실패했을 때의 오류 종류. 사람이 판단할 수 있는 종류만 밝힘.
/// `std::io::Error`가 밝히는 파일 경로는 미포함.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveIoKind {
    /// 임시 파일에 쓰는 중에 실패함.
    TempWrite,
    /// 기존 파일을 교체하는 중에 실패함.
    Replace,
    /// 파일을 읽는 중에 실패함.
    Read,
    /// 쓰기 권한이 없음.
    PermissionDenied,
    /// 파일·폴더가 없음.
    NotFound,
    /// 같은 이름의 파일이 이미 있음.
    AlreadyExists,
    /// 파일이 다른 프로세스에 열려 있어 교체를 못 함(Windows 공유 위반).
    SharingViolation,
    /// 그 밖의 실패.
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
