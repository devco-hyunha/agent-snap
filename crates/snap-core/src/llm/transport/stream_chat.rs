//! 재시도 고리(스트림 열기까지 반복).
//!
//! 전송 한 번을 여러 번 돌려 스트림을 만들 때까지 대기·재시도. 판단은 `retry_loop`, 여기는 고리·대기만.

use std::time::{Duration, Instant, SystemTime};

use crate::llm::client::{CancellationToken, ChatRequest, ChatStream, LlmClient, TransportError};
use crate::llm::retry::retry_loop::{LoopOutcome, judge_once};

use super::OpenAiCompatClient;

impl LlmClient for OpenAiCompatClient {
    async fn stream_chat(
        &self,
        request: ChatRequest,
        cancel: CancellationToken,
    ) -> Result<ChatStream, TransportError> {
        // 호출 전 취소를 바로 알림.
        if cancel.is_cancelled() {
            return Err(TransportError::Cancelled);
        }

        // 재시도 고리는 이 한 층에만 둠. 스트림을 만든 뒤 끊김은 재시도 대상이 아님.
        let settings = self.retry_settings();
        let request_started = Instant::now();
        let mut attempt = 1u32;
        loop {
            match self.send_once(&request, &cancel).await {
                Ok(response) => {
                    return Ok(self.open_stream(response, request_started, cancel));
                }
                Err(failure) => {
                    let outcome = judge_once(
                        failure.error,
                        failure.retry_after.as_deref(),
                        attempt,
                        settings,
                        SystemTime::now(),
                        self.unit_sample(),
                    );
                    match outcome {
                        LoopOutcome::GiveUp(error) => return Err(error),
                        LoopOutcome::Retry { wait } => {
                            self.sleep_before_retry(wait, &cancel).await?;
                            attempt += 1;
                        }
                    }
                }
            }
        }
    }
}

impl OpenAiCompatClient {
    /// 재시도 전 대기. 0이면 즉시 지나감.
    ///
    /// 대기는 취소와 경합. 취소를 먼저 맞으면 취소 오류.
    async fn sleep_before_retry(
        &self,
        wait: Duration,
        cancel: &CancellationToken,
    ) -> Result<(), TransportError> {
        if wait.is_zero() {
            return Ok(());
        }
        tokio::select! {
            () = cancel.cancelled() => Err(TransportError::Cancelled),
            () = tokio::time::sleep(wait) => Ok(()),
        }
    }
}
