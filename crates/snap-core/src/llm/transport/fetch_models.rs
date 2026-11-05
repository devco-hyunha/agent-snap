//! `GET {base_url}/models` 전송 한 번 책임입니다(02-7 작업 2·3).
//!
//! 채팅 전송과 같은 base_url에 경로만 바꿔 붙인다(새 prefix를 잇지 않는다 — `.../v1/v1` 금지, spec).
//! 인증은 02-5와 같고(키가 있으면 Bearer, 없으면 헤더 없음), 재시도 고리는 쓰지 않는다(D14).
//! 시간 값은 02-5 설정을 그대로 쓰고(연결은 HTTP 클라이언트, 헤더 대기는 `header_wait`,
//! 본문 읽기는 `stream_idle`), 목록 JSON 해석과 형식 오류 종류는 `codec::models_list`가 맡는다(SRP).

use crate::llm::client::CancellationToken;
use crate::llm::client::TimeoutSegment;
use crate::llm::client::TransportError;
use crate::llm::codec::models_list::ModelsListIds;
use crate::llm::codec::models_list::{new_empty_body_error, parse_models_ids};

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// `GET {base_url}/models`로 서버가 알려 준 모델 id 목록을 한 번만 조회합니다.
    ///
    /// 재시도는 없고(실패를 바로 알림, D14), 응답은 참조 값으로만 씁니다(설정 `models`와 자동 동기화 금지).
    /// 헤더를 받기 전에 실패하면 취소·시간 초과·연결 실패·서버 상태를 02-5와 같은 오류로 알립니다.
    pub async fn list_models(
        &self,
        cancel: CancellationToken,
    ) -> Result<Vec<String>, TransportError> {
        // 호출 전 취소를 바로 알립니다(채팅 전송과 같은 규격).
        if cancel.is_cancelled() {
            return Err(TransportError::Cancelled);
        }

        let url = format!("{}/models", self.base_url.reveal());
        let builder = self.http_client.get(url);
        let response = self.send_headers(builder, &cancel).await?;
        // 본문 읽기와 해석은 이 파일의 목록 전용 경로를 씁니다(채팅 스트림과 분리).
        self.fetch_models_body(response, &cancel).await
    }

    /// 정상 응답의 목록 본문을 읽어 id 목록으로 바꿉니다.
    ///
    /// 본문 읽기는 채팅 오류 본문 읽기와 같은 유휴·취소 경합을 따르고, 200 본문이 비면 JSON을
    /// 읽지 못한 것이므로 형식 오류로 알립니다. `data`가 없거나 배열이 아니거나 항목의 `id`가
    /// 문자열이 아니면 그 자리에서 목록 조회가 실패한 것으로 봅니다(spec 38~39행).
    async fn fetch_models_body(
        &self,
        response: reqwest::Response,
        cancel: &CancellationToken,
    ) -> Result<Vec<String>, TransportError> {
        if !response.status().is_success() {
            // 재시도 고리를 거치지 않으니 상태→오류 바꾸기도 한 번뿐입니다.
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
