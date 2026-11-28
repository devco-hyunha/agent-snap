//! 이 요청이 알고 있는 키·엔드포인트 호스트 문자열. 2차 가리기 대상만 담음.

use crate::config::secret_string::SecretString;

/// 전송이 알고 있는 비밀 문자열 두 종. 키와 엔드포인트 주소.
///
/// 값은 이 타입 안에서만 쓰고, 오류에는 담지 않는다.
#[derive(Debug, Clone)]
pub struct KnownSecrets {
    /// 요청에 쓰는 인증 키. 없으면 인증 헤더를 보내지 않는다.
    key: Option<SecretString>,

    /// 요청 주소. `/v1`까지 포함한 원문.
    base_url: SecretString,
}

impl KnownSecrets {
    /// 키와 엔드포인트 주소로 2차 가리기 대상 만들기.
    pub fn new(key: Option<SecretString>, base_url: &str) -> Self {
        Self {
            key: key.clone(),
            base_url: SecretString::new(base_url.to_owned()),
        }
    }

    /// 이 전송이 알고 있는 인증 키.
    pub const fn key(&self) -> Option<&SecretString> {
        self.key.as_ref()
    }

    /// 이 전송이 알고 있는 엔드포인트 주소 원문.
    pub fn base_url(&self) -> &str {
        self.base_url.expose()
    }
}
