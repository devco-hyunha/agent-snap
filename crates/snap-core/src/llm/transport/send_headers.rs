//! 채팅 전송과 모델 목록 조회가 같이 쓰는 인증 헤더·헤더 대기 경로.
//!
//! 키 있을 때만 Bearer. 보내기·헤더 대기·취소·연결 실패를 한 자리에서 경합. 분류는 `connect_classify`.

use crate::llm::client::{CancellationToken, TimeoutSegment, TransportError};
use crate::llm::transport::connect_classify::classify_connect_error;

use super::OpenAiCompatClient;

impl OpenAiCompatClient {
    /// 키가 있을 때만 붙이는 `Authorization: Bearer` 헤더 값.
    ///
    /// 키 원문은 버리고 조립 실패는 클라이언트 조립 오류로만 알림. 민감 헤더로 표시.
    pub(super) fn auth_header(
        &self,
    ) -> Result<Option<reqwest::header::HeaderValue>, TransportError> {
        let Some(api_key) = &self.api_key else {
            return Ok(None);
        };
        let mut value =
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key.reveal()))
                .map_err(|_| TransportError::ClientBuild)?;
        value.set_sensitive(true);
        Ok(Some(value))
    }

    /// 요청에 인증 헤더를 붙이고 응답 헤더까지 받아 옴.
    ///
    /// 채팅·목록 조회가 같은 경로. 헤더 값은 라이브러리 로그에서도 가림.
    pub(super) async fn send_headers(
        &self,
        builder: reqwest::RequestBuilder,
        cancel: &CancellationToken,
    ) -> Result<reqwest::Response, TransportError> {
        let mut builder = builder;
        if let Some(value) = self.auth_header()? {
            builder = builder.header(reqwest::header::AUTHORIZATION, value);
        }
        self.wait_for_headers(builder, cancel).await
    }

    /// 응답 헤더를 정해진 시간 안에 수신. 보내기·헤더 대기·취소를 한 자리에서 경합.
    pub(super) async fn wait_for_headers(
        &self,
        builder: reqwest::RequestBuilder,
        cancel: &CancellationToken,
    ) -> Result<reqwest::Response, TransportError> {
        let send = builder.send();
        // 보내기 완료 대기. 이 자리에서 취소를 함께 확인(취소 시 연결 정리는 아래 스트림에서).
        tokio::select! {
            biased;
            () = cancel.cancelled() => Err(TransportError::Cancelled),
            () = tokio::time::sleep(self.http.header_wait_seconds) => Err(self.timeout_error(TimeoutSegment::HeaderWait)),
            result = send => result.map_err(|e| self.connect_error(&e)),
        }
    }

    /// 헤더 전 연결 실패를 세부 6종으로 분류.
    ///
    /// 라이브러리가 주소를 오류 문자열에 넣을 수 있어 원문 텍스트는 버리고,
    /// Display·Debug에는 종류와 연결 설정 초 값만 담음.
    fn connect_error(&self, error: &reqwest::Error) -> TransportError {
        TransportError::ConnectFailed {
            kind: classify_connect_error(error),
            connect_seconds: self.http.connect_seconds.as_secs(),
        }
    }
}
