//! `OpenAiCompatClient`의 Debug 표기.
//!
//! 표기 형식은 생성·설정과 다른 이유로 바뀔 수 있어 따로 둠. 키·주소는 이미 가려진 값만 보임.

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
