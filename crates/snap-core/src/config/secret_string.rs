//! 비밀 값 래퍼. Debug 출력에는 값을 내지 않는 자리.

use crate::locale::config::SECRET_REDACTED;

/// 비밀 문자열. Debug 출력에서는 값 대신 가렸음 표시만.
/// 값 비교는 래퍼끼리 같은 값인지 볼 때만 쓴다.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretString(String);

impl SecretString {
    /// 문자열을 비밀 값으로 감싼 래퍼.
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    /// 실제 값. 전송·저장이 필요한 지점에서만 쓰는 창구.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecretString({SECRET_REDACTED})")
    }
}

impl<'de> serde::Deserialize<'de> for SecretString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Self(value))
    }
}
