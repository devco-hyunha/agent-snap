//! 비밀 값 래퍼: Debug와 Display에서 값을 가리고, 원문은 명시적인 함수로만 꺼내게 합니다.

use std::fmt;

/// 비밀 값(예: API 키)을 담는 래퍼입니다.
///
/// `Debug`와 `Display`로 찍으면 값 대신 `가림` 표기가 나오고, 원문은 [`SecretValue::reveal`]로만
/// 꺼냅니다. 값이 필요한 자리에 이 타입을 써서 실수로 출력에 값이 남는 경로를 차단합니다.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretValue {
    value: String,
}

impl SecretValue {
    /// 문자열 값으로 비밀 값 래퍼를 만듭니다.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// 원문을 꺼냅니다. 호출한 자리에서 값이 노출되는 것을 명시적으로 드러내는 함수이므로,
    /// 출력에 직접 넣는 데 쓰지 않습니다.
    pub fn reveal(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for SecretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretValue(가림)")
    }
}

impl fmt::Display for SecretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "가림")
    }
}

#[cfg(test)]
mod tests;
