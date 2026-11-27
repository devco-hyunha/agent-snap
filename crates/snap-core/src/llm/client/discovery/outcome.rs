//! 자동 발견 결과 타입 자리. 필드와 표시 구현만 담음.

use crate::config::secret_string::SecretString;
use crate::llm::client::kind::EndpointKind;

/// 자동 발견 결과. 채운 주소 또는 발견 실패.
///
/// 발견 실패는 오류가 아니라 "채우지 못함"으로 본다. 재시도 요청을 늘리지 않는다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryOutcome {
    /// 기본 주소로 연결이 열려 주소를 채움.
    Found {
        /// 채워진 엔드포인트 주소. `/v1`까지 포함한 기본 주소.
        /// Debug 출력에 주소가 나가지 않게 비밀 값 래퍼로 감쌈.
        base_url: SecretString,
    },

    /// 발견하지 못함. 대상 종류만 담음.
    NotFound {
        /// 발견을 시도한 엔드포인트 종류.
        kind: EndpointKind,
    },
}

impl DiscoveryOutcome {
    /// 채워진 주소. 발견 실패면 없음.
    pub fn filled(&self) -> Option<&str> {
        match self {
            Self::Found { base_url } => Some(base_url.expose()),
            Self::NotFound { .. } => None,
        }
    }

    /// 발견 실패인지.
    pub const fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound { .. })
    }
}

impl std::fmt::Display for DiscoveryOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Found { base_url } => write!(f, "{}", base_url.expose()),
            Self::NotFound { kind } => write!(f, "{kind}"),
        }
    }
}
