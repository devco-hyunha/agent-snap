//! 조립 결과 타입 자리. 필드만 담음.
//!
//! reqwest 타입은 이 타입 어디에도 들어가지 않는다. 전송은 재시도 래퍼로만 감긴 채 나온다.

use crate::llm::client::retry::RetryClient;
use crate::llm::client::transport::HttpTransport;
use crate::llm::retry::SystemRandomSource;

/// 설정에서 고른 대상으로 조립한 클라이언트와 서버에 보낼 모델 식별 값 한 짝.
#[derive(Debug, Clone)]
pub struct ClientBundle {
    /// 재시도 래퍼가 감긴 클라이언트. 에이전트 루프는 `LlmClient` 트레이트로만 본다.
    pub client: RetryClient<HttpTransport, SystemRandomSource>,

    /// 서버에 보내는 모델 식별 값. 설정 `models[].id`를 그대로 씀.
    pub model_id: String,
}
