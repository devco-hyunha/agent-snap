//! 서버 상태와 시간 초과를 전송 오류로 바꾸는 만들기.
//!
//! 채팅·목록이 같은 경로. 채팅은 비정상 상태를 재시도 대상으로, 목록은 한 번만. 시간 초과는 구간·초만 담음.

use crate::llm::client::{CancellationToken, TimeoutSegment, TransportError};
use crate::llm::error::redact::summarize_stream_error;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 구간 이름과 설정 초 값으로 시간 초과 오류를 만듦.
    pub(super) fn timeout_error(&self, segment: TimeoutSegment) -> TransportError {
        let seconds = match segment {
            TimeoutSegment::HeaderWait => self.http.header_wait_seconds,
            TimeoutSegment::StreamIdle => self.http.stream_idle_seconds,
        };
        TransportError::Timeout {
            segment,
            seconds: seconds.as_secs(),
        }
    }

    /// 서버가 정상이 아닌 상태로 준 응답을 오류로 바꿈.
    ///
    /// 본문은 어림·확인 가리기. 유휴·취소와 경합. 채팅·목록이 같은 만들기.
    pub(super) async fn server_status_error(
        &self,
        response: reqwest::Response,
        cancel: &CancellationToken,
    ) -> TransportError {
        let status = response.status().as_u16();
        let body = tokio::select! {
            result = response.bytes() => {
                match result {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                    Err(error) => summarize_stream_error(&error.to_string()),
                }
            }
            // 취소는 상태 요약 없이 취소 오류로 바로 끝냄.
            () = cancel.cancelled() => return TransportError::Cancelled,
            () = tokio::time::sleep(self.http.stream_idle_seconds) => "서버가 오류 본문을 보내지 않았습니다".to_owned(),
        };
        TransportError::ServerStatus {
            status,
            body_summary: self.scrub().describe(&body),
            // 판단을 거치지 않은 단일 요청 실패에는 포기 이유가 없음.
            give_up: None,
        }
    }
}
