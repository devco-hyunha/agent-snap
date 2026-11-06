//! 키 정하기: 엔드포인트 이름으로 자격 파일에서 비밀 값을 찾습니다(DECISIONS D10).

use super::types::EndpointConfig;
use crate::credentials::Credentials;
use crate::secrets::SecretValue;

impl EndpointConfig {
    /// 이 엔드포인트가 쓸 키를 자격 파일에서 이름으로 찾습니다.
    ///
    /// 이름이 없거나 값이 빈 문자열이거나 자격 파일이 없으면 `None`('키 없음')이며, 이때
    /// 요청은 인증 헤더 없이 갑니다.
    pub fn api_key<'a>(&self, credentials: &'a Credentials) -> Option<&'a SecretValue> {
        credentials.find_key(&self.name)
    }
}
