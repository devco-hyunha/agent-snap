//! `GET {base_url}/models` 전송 한 번.
//!
//! 채팅과 같은 base_url에 경로만 바꿈. 키 있으면 Bearer. 재시도 고리 없음. 목록 해석은 `models_list`.

use crate::llm::client::CancellationToken;
use crate::llm::client::TimeoutSegment;
use crate::llm::client::TransportError;
use crate::llm::selection::models_list::ModelsListIds;
use crate::llm::selection::models_list::{new_empty_body_error, parse_models_ids};

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// `GET {base_url}/models`로 모델 id 목록을 한 번만 조회.
    ///
    /// 재시도 없음. 응답은 참조 값으로만 씀. 헤더 전 실패는 채팅과 같은 오류로 알림.
    pub async fn list_models(
        &self,
        cancel: CancellationToken,
    ) -> Result<Vec<String>, TransportError> {
        // 호출 전 취소를 바로 알림.
        if cancel.is_cancelled() {
            return Err(TransportError::Cancelled);
        }

        let url = format!("{}/models", self.base_url.reveal());
        let builder = self.http_client.get(url);
        let response = self.send_headers(builder, &cancel).await?;
        // 본문 읽기·해석은 목록 전용 경로(채팅 스트림과 분리).
        self.fetch_models_body(response, &cancel).await
    }

    /// 정상 응답의 목록 본문을 읽어 id 목록으로 바꿈.
    ///
    /// 본문 읽기는 채팅과 같은 유휴·취소 경합. 빈 본문·형식 위반은 형식 오류로 알림.
    async fn fetch_models_body(
        &self,
        response: reqwest::Response,
        cancel: &CancellationToken,
    ) -> Result<Vec<String>, TransportError> {
        if !response.status().is_success() {
            // 재시도 고리를 거치지 않으니 상태→오류 바꾸기도 한 번뿐.
            return Err(self.server_status_error(response, cancel).await);
        }
        let bytes = tokio::select! {
            result = response.bytes() => {
                match result {
                    Ok(bytes) => bytes,
                    Err(_) => return Err(TransportError::BodyRead),
                }
            }
            () = cancel.cancelled() => return Err(TransportError::Cancelled),
            () = tokio::time::sleep(self.http.stream_idle_seconds) => {
                return Err(self.timeout_error(TimeoutSegment::StreamIdle));
            }
        };
        // 본문이 비면 JSON으로 읽지 못한 것으로 형식 오류로 냅니다(빈 목록 성공 금지).
        if bytes.is_empty() {
            return Err(TransportError::ModelsListFormat(new_empty_body_error()));
        }
        let text = String::from_utf8_lossy(&bytes);
        let ModelsListIds(ids) = parse_models_ids(&text)?;
        Ok(ids)
    }
}
