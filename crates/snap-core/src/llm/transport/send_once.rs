//! 채팅 요청 전송 한 번.
//!
//! POST 조립·1회 보내기·비정상 응답을 실패 봉지로 바꿈. 헤더는 `send_headers`, 오류 만들기는 `status_error`.

use crate::llm::client::{CancellationToken, ChatRequest, TransportError};
use crate::llm::request_body::build_chat_completions_body;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 응답 헤더까지 받아 오는 채팅 전송 한 번.
    ///
    /// 비정상이면 오류로 바꿈. 실패 봉지에는 `Retry-After`만 담고 공개 오류에는 넘기지 않음.
    pub(super) async fn send_once(
        &self,
        request: &ChatRequest,
        cancel: &CancellationToken,
    ) -> Result<reqwest::Response, SendFailure> {
        let url = format!("{}/chat/completions", self.base_url.reveal());
        let body = build_chat_completions_body(self.kind, request);
        let builder = self.http_client.post(url).json(&body);
        // 헤더 붙이기와 헤더 대기는 목록 조회와 같은 경로.
        let response = self
            .send_headers(builder, cancel)
            .await
            .map_err(SendFailure::from)?;
        if !response.status().is_success() {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned);
            return Err(SendFailure {
                error: self.server_status_error(response, cancel).await,
                retry_after,
            });
        }
        Ok(response)
    }
}

/// 재시도 판단에 넘기는 전송 실패 봉지.
///
/// 방금 실패한 오류와 `Retry-After` 원문만 담음. 공개 오류에는 남지 않음.
pub(super) struct SendFailure {
    /// 방금 실패한 전송 오류.
    pub(super) error: TransportError,
    /// `Retry-After` 헤더 원문. 없거나 읽을 수 없으면 `None`.
    pub(super) retry_after: Option<String>,
}

/// 헤더를 받기 전 단계 실패 봉지. 응답이 없으니 헤더 값도 없음.
impl From<TransportError> for SendFailure {
    fn from(error: TransportError) -> Self {
        Self {
            error,
            retry_after: None,
        }
    }
}
