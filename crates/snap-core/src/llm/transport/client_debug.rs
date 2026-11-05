//! `OpenAiCompatClient`의 Debug 표기 책임입니다.
//!
//! 표기 형식은 클라이언트 생성·설정 관리와 서로 다른 이유로 바뀔 수 있어 따로 둡니다(작업 07 SRP 분리).
//! 키·주소는 필드가 SecretValue 타입이라 Debug에서 이미 가려진 값만 보입니다.

use std::fmt;

use super::client::OpenAiCompatClient;

impl fmt::Debug for OpenAiCompatClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAiCompatClient")
            .field("base_url", &self.base_url)
            .field("kind", &self.kind)
            .field("api_key", &self.api_key)
            .field("http", &self.http)
            .finish_non_exhaustive()
    }
}
