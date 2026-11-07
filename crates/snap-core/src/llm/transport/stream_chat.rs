//! 재시도 고리(스트림 열기까지 반복) 책임입니다.
//!
//! 전송 한 번을 여러 번 돌려 스트림을 만들 때까지 기다리고 다시 시도합니다(02-6).
//! 판단 자체는 `retry_loop`가 정하고, 이 파일은 고리 돌기와 재시도 전 대기만 합니다.

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
        // 호출 전 취소를 바로 알립니다.
        if cancel.is_cancelled() {
            return Err(TransportError::Cancelled);
        }

        // 재시도 고리는 이 한 층에만 둡니다(상위 루프에는 손대지 않음, D13).
        // 스트림을 만든 뒤 끊기는 일은 재시도 대상이 아닙니다(02-6 spec 13행).
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
    /// 재시도 전 대기입니다. 대기가 0이면 즉시 지나갑니다(AC 1의 `Retry-After` 0초 포함).
    ///
    /// 대기는 취소와 경합합니다(cancel-safe, tokio-util 근거). 취소를 먼저 맞으면 취소 오류를 냅니다.
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
