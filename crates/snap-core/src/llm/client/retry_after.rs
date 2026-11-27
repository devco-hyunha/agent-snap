//! 상태 오류가 담는 `Retry-After` 헤더 값 타입 자리. Debug에는 값 대신 가렸음 표시만.

use crate::locale::config::SECRET_REDACTED;

/// 서버가 준 `Retry-After` 헤더 값. 2차 가리기를 지난 값만 담음.
///
/// 해석은 02-4의 `interpret_retry_after`가 맡고, 이 타입은 값 보관만 함.
#[derive(Clone, PartialEq, Eq)]
pub struct RetryAfterHeader(String);

impl RetryAfterHeader {
    /// 2차 가리기를 지난 헤더 값.
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    /// 실제 헤더 값. 02-4 판단에 넘기는 지점에서만 쓰는 창구.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for RetryAfterHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RetryAfterHeader({SECRET_REDACTED})")
    }
}
