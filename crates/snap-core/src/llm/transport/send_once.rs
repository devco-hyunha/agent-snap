//! 채팅 요청 전송 한 번 책임입니다(02-5).
//!
//! POST 조립과 1회 보내기, 정상이 아닌 응답을 재시도 판단 실패 봉지로 바꾸는 읽기까지 다룬다.
//! 인증 헤더·헤더 대기는 `send_headers`, 상태와 시간 초과를 오류로 바꾸는 만들기는 `status_error`가 맡는다
//! (채팅·목록이 같은 경로를 쓰므로 파일로 나눴다 — dev-principles 2절 SRP).
//! 재시도 판단 반복과 대기·스트림 열기는 각각 `retry_loop`·`stream_chat`·`open_stream`이 맡는다.

use crate::llm::client::{CancellationToken, ChatRequest, TransportError};
use crate::llm::request_body::build_chat_completions_body;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 응답 헤더까지 받아 오는 채팅 전송 한 번입니다.
    ///
    /// 정상 상태가 아니면 오류로 바꿉니다. 실패 봉지에는 응답이 준 `Retry-After` 헤더 값만 담습니다.
    /// 헤더 원문은 키·호스트와 달리 공개 오류 타입에는 넘기지 않고 판단에만 씁니다(02-6 설계 요약).
    pub(super) async fn send_once(
        &self,
        request: &ChatRequest,
        cancel: &CancellationToken,
    ) -> Result<reqwest::Response, SendFailure> {
        let url = format!("{}/chat/completions", self.base_url.reveal());
        let body = build_chat_completions_body(self.kind, request);
        let builder = self.http_client.post(url).json(&body);
        // 헤더 붙이기와 헤더 대기는 모델 목록 조회와 같은 경로를 씁니다(02-7).
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

/// 재시도 판단에 넘기는 전송 실패 봉지입니다.
///
/// 방금 실패한 오류와, 응답이 준 `Retry-After` 헤더 원문 값만 담습니다.
/// 헤더 원문은 판단이 소비하고 공개 오류에는 남지 않습니다(02-6 spec 33행).
pub(super) struct SendFailure {
    /// 방금 실패한 전송 오류입니다.
    pub(super) error: TransportError,
    /// `Retry-After` 헤더 원문입니다. 헤더가 없거나 읽을 수 없으면 `None`입니다.
    pub(super) retry_after: Option<String>,
}

/// 헤더를 받기 전 단계 실패 봉지입니다. 응답이 없으니 헤더 값도 없습니다.
impl From<TransportError> for SendFailure {
    fn from(error: TransportError) -> Self {
        Self {
            error,
            retry_after: None,
        }
    }
}
