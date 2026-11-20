//! 폴백 연결 지점 표기 타입만 담당하는 자리

use std::fmt::{self, Display, Formatter};

/// 폴백 연결 지점 커넥터. 오류를 소비하는 자리를 나타내는 표기 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FallbackConnector;

impl FallbackConnector {
    /// 오류를 소비하는 자리임을 밝히는 고정 표기 값
    pub const FALLBACK_CONNECTOR: &'static str = "03-4 폴백 연결 지점";
}

impl Display for FallbackConnector {
    /// 고정 표기 값을 그대로 출력
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(Self::FALLBACK_CONNECTOR)
    }
}
