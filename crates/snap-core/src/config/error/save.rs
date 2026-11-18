//! 설정 파일·자격 파일 쓰기가 실패한 이유만 담는 열거형.

use std::fmt;

use super::super::save::error::{SaveIoKind, SaveIoTarget};

/// 설정 파일과 자격 파일을 쓰다가 실패한 오류.
///
/// 저장 규칙이 바뀌는 이유만 바뀌며, 읽기·검증 이유는 각각 `error/reading.rs`·
/// `error/validation.rs`가 맡음. Display·Debug에 경로 전체·키 값·base_url을 담지 않음.
#[derive(thiserror::Error)]
pub enum SaveError {
    /// 엔드포인트 이름 바꾸기의 설정 파일 교체 실패. 자격 파일 키 이름은 이미
    /// 바뀐 상태이고, 교체 실패 원인만 담음.
    #[error("엔드포인트 이름 바꾸기 실패: 자격 파일 키 이름은 이미 바뀌었습니다")]
    EndpointRenameAfterKeySide {
        /// 교체 실패 원인. 원인을 모를 때는 `None`이고 문구만 노출.
        source: Option<std::io::Error>,
    },
    /// 엔드포인트 삭제의 설정 파일 교체 실패. 자격 파일 키는 이미 지워진 상태이고,
    /// 교체 실패 원인만 담음.
    #[error("엔드포인트 삭제 실패: 자격 파일 키는 이미 지워졌습니다")]
    EndpointRemoveAfterKeySide {
        /// 교체 실패 원인. 원인을 모를 때는 `None`이고 문구만 노출.
        source: Option<std::io::Error>,
    },
    /// 저장이 파일 쓰기에 실패한 상태. 대상 파일 종류와 오류 종류만 Display·Debug에 남김.
    /// 담지 않는 것: 경로 전체, 키 값, base_url.
    #[error("{target} {kind}")]
    SaveIo {
        /// 어떤 파일 저장이 실패했는지만 밝힘. 경로·파일 이름은 밝히지 않음.
        target: SaveIoTarget,
        /// 어떤 종류의 실패인지만 밝힘.
        kind: SaveIoKind,
    },
    /// 저장이 바꿀 엔드포인트가 설정 파일에 없는 상태. 엔드포인트 이름만 담음.
    /// 담지 않는 것: 주소, 키.
    #[error("저장이 바꿀 엔드포인트 `{endpoint}`가 설정 파일에 없습니다")]
    MissingEndpoint {
        /// trim한 엔드포인트 이름.
        endpoint: String,
    },
}

impl SaveError {
    /// 저장이 바꿀 엔드포인트가 없을 때의 오류. 엔드포인트 이름만 담음.
    pub(super) fn missing_endpoint(endpoint: &str) -> Self {
        Self::MissingEndpoint {
            endpoint: endpoint.to_owned(),
        }
    }
}

impl fmt::Debug for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 함.
        write!(f, "SaveError({})", self)
    }
}
