//! 서버 상태와 시간 초과를 전송 오류로 바꾸는 만들기를 한 파일에 모았습니다(02-5·02-7).
//!
//! 채팅 전송과 모델 목록 조회가 같은 경로를 쓴다. 채팅은 상태 코드가 정상이 아닌 4xx·5xx
//! 응답을 오류로 바꿔서 재시도 대상으로 넘기고(`send_once`), 목록 조회는 한 번짜리라 재시도
//! 대상에 넣지 않는다(`fetch_models`).
//! 시간 초과는 구간 이름(`TimeoutSegment`)과 설정 초 값만 담아 키·주소가 드러나지 않게 한다.

use crate::llm::client::{CancellationToken, TimeoutSegment, TransportError};
use crate::llm::error::redact::summarize_stream_error;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 구간 이름과 설정 초 값으로 시간 초과 오류를 만든다.
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

    /// 서버가 정상이 아닌 상태로 준 응답을 오류로 바꾼다.
    ///
    /// 본문은 어림 가리기 뒤 알고 있는 키·호스트로 한 번 더 가린다(D28). 읽기도 유휴·취소와
    /// 경합해 취소되면 취소 오류를 내고, 헤더는 보냈지만 본문을 끝까지 안 보내는 서버에 멈춰 있지 않는다.
    /// 채팅 전송과 모델 목록 조회가 같은 만들기를 쓴다(상태→오류 봉지 한 지식, 02-7).
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
            // 취소는 상태 코드 요약을 내려주지 않고 취소 오류로 바로 끝낸다.
            () = cancel.cancelled() => return TransportError::Cancelled,
            () = tokio::time::sleep(self.http.stream_idle_seconds) => "서버가 오류 본문을 보내지 않았습니다".to_owned(),
        };
        TransportError::ServerStatus {
            status,
            body_summary: self.scrub().describe(&body),
            // 판단을 거치지 않은 단일 요청 실패에는 포기 이유가 없다.
            give_up: None,
        }
    }
}
