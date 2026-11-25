//! 가린 서버 오류 요약 타입 자리. Debug 출력에는 값 대신 가렸음 표시만.

use crate::locale::config::SECRET_REDACTED;

/// 가린 서버 오류 요약. Debug 출력에는 값 대신 가렸음 표시만.
#[derive(Clone, PartialEq, Eq)]
pub struct RedactedSummary(String);

impl RedactedSummary {
    /// 가린 요약 값.
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    /// 실제 요약. 화면에 띄우는 지점에서만 쓰는 창구.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for RedactedSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RedactedSummary({SECRET_REDACTED})")
    }
}

impl std::fmt::Display for RedactedSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
