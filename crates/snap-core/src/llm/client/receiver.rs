//! 02-3 복원 상태를 클라이언트 수신자로 연결하는 자리. 재구현 없이 `StreamState`만 감쌈.

use std::time::Instant;

use crate::llm::SseError;
use crate::llm::StreamState;
use crate::llm::client::error::TransportError;
use crate::llm::client::stream::StreamOutcome;
use crate::llm::client::token_times::TokenTimes;
use crate::llm::client::traits::StreamReceiver;

/// 02-3 복원 상태를 그대로 쓰는 스트림 수신자.
///
/// 첫 실패 이후에는 같은 복원 상태로 결과를 내지 않음.
#[derive(Debug)]
pub struct RestoreReceiver {
    /// 02-3 복원 상태. SSE·델타 규칙은 이쪽 정본만 사용.
    state: StreamState,

    /// 요청 시작과 토큰 시각 기록.
    times: TokenTimes,

    /// 첫 복원 실패. 성공으로 오해할 결과를 막는 기준.
    failure: Option<SseError>,

    /// 스트림이 한 번 끝났는지.
    is_closed: bool,
}

impl RestoreReceiver {
    /// 요청 시작 시각만 받는 수신자 열기. 복원 상태는 기본값으로 채움.
    pub fn new(request_start: Instant) -> Self {
        Self {
            state: StreamState::new(),
            times: TokenTimes::new(request_start),
            failure: None,
            is_closed: false,
        }
    }

    /// 첫 복원 실패를 남기고 전송 오류로 냄. 같은 실패를 다시 내는 기준.
    fn fail(&mut self, error: SseError) -> TransportError {
        self.failure = Some(error.clone());
        TransportError::stream(error)
    }

    /// 받은 첫 복원 실패를 전송 오류로 옮김.
    fn stored_error(&self) -> Option<TransportError> {
        let error = self.failure.clone()?;
        Some(TransportError::stream(error))
    }
}

impl StreamReceiver for RestoreReceiver {
    /// 바이트 조각을 02-3 복원에 넣고 합침.
    fn push(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
        if self.is_closed {
            return Err(TransportError::closed());
        }
        if let Some(error) = self.stored_error() {
            return Err(error);
        }
        self.state.push(bytes).map_err(|error| self.fail(error))?;
        let delta = self.state.delta();
        self.times
            .record(delta.content.len() + delta.reasoning.len());
        Ok(())
    }

    /// 스트림 끝. 마지막까지 합친 결과 반환.
    fn finish(&mut self) -> Result<StreamOutcome, TransportError> {
        self.is_closed = true;
        if let Some(error) = self.stored_error() {
            return Err(error);
        }
        self.state.finish().map_err(|error| self.fail(error))?;
        Ok(StreamOutcome {
            delta: self.state.delta(),
            timings: self.times.timings(),
        })
    }
}
