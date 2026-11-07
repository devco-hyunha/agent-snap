//! 시간 초과 구간 표시 타입입니다.

use std::fmt;

/// 시간 초과가 난 구간입니다. 어느 제한을 넘겼는지 표시하는 값만 담고, 값 초는 비밀이 아닙니다(D14).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeoutSegment {
    /// 응답 헤더를 기다리는 구간입니다.
    HeaderWait,
    /// 스트림에서 다음 조각을 기다리는 구간입니다.
    StreamIdle,
}

impl fmt::Display for TimeoutSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::HeaderWait => "헤더 대기",
            Self::StreamIdle => "스트림 유휴",
        };
        f.write_str(name)
    }
}
